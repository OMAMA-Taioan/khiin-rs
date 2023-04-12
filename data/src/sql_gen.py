from textwrap import dedent
import csv_parsers as csv
import lomaji
import argparse
import itertools
import locale
import sqlite3
import sys
from typing import List, Tuple

from tables import Conversion, Frequency, SqlInputSeq

locale.setlocale(locale.LC_ALL, '')

# SQL Tables
FREQ = "frequency"
FREQ_INPUT = "input"
F_FREQ = "count"
FREQ_CHID = "chhan_id"

CONV = "conversions"
CONV_INID = "input_id"
CONV_OUT = "output"
CONV_WT = "weight"
CONV_CAT = "category"
CONV_ANNO = "annotation"

INP = "input_sequences"
INP_INID = "input_id"
INP_NUM = "numeric"
INP_TEL = "telex"
INP_N = "n_syls"
INP_P = "p"

META = "metadata"
META_KEY = "key"
META_VAL = "value"

UGRAM = "unigram_freq"
UGRAM_GRAM = "gram"
UGRAM_N = "count"

BGRAM = "bigram_freq"
BGRAM_L = "lgram"
BGRAM_R = "rgram"
BGRAM_N = "count"

# SQL Views
CL = "conversion_lookups"
CL_NUM = "numeric"
CL_TEL = "telex"
CL_INP = "input"
CL_INID = "input_id"
CL_OUT = "output"
CL_WT = "weight"
CL_CAT = "category"
CL_ANNO = "annotation"

spinner = itertools.cycle(['-', '/', '|', '\\'])


def show_progress():
    global spinner
    sys.stdout.write(next(spinner))
    sys.stdout.flush()
    sys.stdout.write('\b')
    return 0

##############################################################################
#
# Data validation and collection
#
##############################################################################


def get_unique(dataset, field_name: str) -> List[str]:
    ret = set()
    for each in dataset:
        ret.add(getattr(each, field_name))
    return list(ret)


def find_common_inputs(freq: List[Frequency],
                       conv: List[Conversion]) -> Tuple[List[Frequency],
                                                        List[Conversion]]:
    unique_freq_inputs = get_unique(freq, 'input')
    unique_conv_inputs = get_unique(conv, 'input')

    freq_has_conv = []
    conv_has_freq = []

    for word in freq:
        if word.input in unique_conv_inputs:
            freq_has_conv.append(word)

    for word in conv:
        if word.input in unique_freq_inputs:
            conv_has_freq.append(word)

    freq = sorted(freq_has_conv, key=csv.freq_sort_key)
    conv = sorted(conv_has_freq, key=csv.conv_sort_key)
    return (freq, conv)


def get_input_sequences(freq: List[Frequency]):
    total_count = sum(row.freq for row in freq)
    input_seqs: List[SqlInputSeq] = []
    for row in freq:
        row_seqs = lomaji.to_input_sequences(row.input)
        n_syls = len(row_seqs)
        for (numeric, telex, n_syls) in row_seqs:
            input_seqs.append(SqlInputSeq(
                row.input, numeric, telex, n_syls, p=row.freq / total_count
            ))
    return input_seqs


def get_extra_syllables(syls, freq, conv):
    ret = set(syls)
    for x in freq:
        for syl in x["input"].split(' '):
            ret.add(lomaji.normalize_loji(syl, True))
    for x in conv:
        for syl in x["input"].split(' '):
            ret.add(lomaji.normalize_loji(syl, True))
    return sorted(list(ret), key=csv.syls_sort_key)

##############################################################################
#
# SQL builder functions
#
##############################################################################


def init_db_sql():
    return dedent(f"""\
        DROP TABLE IF EXISTS "metadata";
        DROP TABLE IF EXISTS "conversions";
        DROP TABLE IF EXISTS "frequency";
        DROP TABLE IF EXISTS "input_sequences";
        DROP TABLE IF EXISTS "syllables";
        DROP INDEX IF EXISTS "unigram_freq_gram_idx";
        DROP TABLE IF EXISTS "unigram_freq";
        DROP INDEX IF EXISTS "bigram_freq_gram_index";
        DROP TABLE IF EXISTS "bigram_freq";

        CREATE TABLE IF NOT EXISTS "metadata" (
            "key"	TEXT,
            "value"	INTEGER
        );

        {Frequency.create_statement()}
        {Conversion.create_statement()}
        {SqlInputSeq.create_statement()}

        CREATE TABLE IF NOT EXISTS "syllables" (
            "input"   TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS "unigram_freq" (
            "gram"	TEXT NOT NULL UNIQUE,
            "n"	INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS "bigram_freq" (
            "lgram"	TEXT,
            "rgram"	TEXT,
            "n"	INTEGER NOT NULL,
            UNIQUE("lgram","rgram")
        );

        CREATE INDEX "conversions_input_id_covering_index" ON "conversions" (
            "input_id",
            "output",
            "weight",
            "category",
            "annotation"
        );

        CREATE INDEX "input_numeric_covering_index" ON "input_sequences" (
            "numeric",
            "input_id"
        );

        CREATE INDEX "input_telex_covering_index" ON "input_sequences" (
            "telex",
            "input_id"
        );

        CREATE INDEX "unigram_gram_index" ON "unigram_freq" (
            "gram"
        );

        CREATE INDEX "bigram_gram_index" ON "bigram_freq" (
            "rgram",
            "lgram"
        );

        DROP VIEW IF EXISTS "input_view";
        CREATE VIEW "input_view" (
            numeric,
            telex,
            input,
            input_id,
            output,
            weight,
            category,
            annotation
        ) as SELECT
            n.numeric,
            n.telex,
            f.input,
            n.input_id,
            c.output,
            c.weight,
            c.category,
            c.annotation
        FROM input_sequences AS n
        JOIN frequency AS f ON f.id = n.input_id
        JOIN conversions AS c ON f.id = c.input_id;

        DROP VIEW IF EXISTS "ngrams";
        CREATE VIEW "ngrams" (
            lgram,
            rgram,
            rgram_count,
            bigram_count
        ) AS SELECT
            b.lgram,
            u.gram,
            u.n AS unigram_count,
            b.n AS bigram_count
        FROM unigram_freq AS u
        LEFT JOIN bigram_freq AS b ON u.gram = b.rgram;
        """)


def inout_lookup_sql(row: SqlInputSeq):
    return dedent(f"""\
        INSERT INTO "input_sequences" \
        ("input_id", "numeric", "telex", "n_syls", "p") \
        SELECT \
        "id", "{row.numeric}", "{row.telex}", "{row.n_syls}", "{row.p}" \
        FROM "frequency" WHERE "input"="{row.input}";""")


def input_sql(data: List[SqlInputSeq]) -> str:
    insert_list = [inout_lookup_sql(row) for row in data]
    sql = '\n'.join(insert_list) + '\n'
    return sql


def syls_sql(data: List[str]):
    sql = 'INSERT INTO "syllables" ("input") VALUES\n'
    values = ',\n'.join([f'("{syl}")' for syl in data])
    sql += values + ';\n'
    return sql


def build_sql(
        freq: List[Frequency],
        conv: List[Conversion],
        inputs: List[SqlInputSeq],
        syls: List[str]):
    sql = dedent("""\
            PRAGMA journal_mode = OFF;
            PRAGMA cache_size = 7500000;
            PRAGMA synchronous = OFF;
            PRAGMA temp_store = 2;
            BEGIN TRANSACTION;
            """)

    sql += init_db_sql()

    sql += Frequency.insert_statement(freq)

    sql += Conversion.insert_statement(conv)

    sql += input_sql(inputs)

    sql += syls_sql(syls) if (len(syls) > 0) else ""

    sql += dedent("""\
            COMMIT;
            PRAGMA journal_mode = WAL;
            PRAGMA cache_size = -2000;
            PRAGMA synchronous = NORMAL;
            PRAGMA temp_store = 0;
            """)

    return sql


def write_sql(sql_file, sql):
    with open(sql_file, 'w', encoding='utf-8') as f:
        f.write(sql)

##############################################################################
#
# SQLite DB builder
#
##############################################################################


def build_symbols_table(db_cur, symbol_tsv):
    db_cur.executescript(dedent("""\
        DROP TABLE IF EXISTS "symbols";
        CREATE TABLE "symbols" (
            "id"           INTEGER PRIMARY KEY,
            "input"        TEXT NOT NULL,
            "output"       TEXT NOT NULL,
            "category"     INTEGER,
            "annotation"   TEXT
        );
        """))
    dat = csv.parse_symbols_tsv(symbol_tsv)
    tuples = list(map(lambda row: (row.input, row.output, row.category), dat))
    db_cur.executemany(dedent("""\
        INSERT INTO "symbols" \
        ("input", "output", "category") \
        VALUES (?, ?, ?);"""),
                       tuples)


def build_emoji_table(db_cur, emoji_csv):
    db_cur.executescript(dedent("""\
        DROP TABLE IF EXISTS "emoji";
        CREATE TABLE "emoji" (
            id INTEGER PRIMARY KEY,
            emoji TEXT NOT NULL,
            short_name TEXT NOT NULL,
            category INTEGER NOT NULL,
            code TEXT NOT NULL
        );
        """))
    dat = csv.parse_emoji_csv(emoji_csv)
    tuples = list(map(lambda row: (row.id, row.emoji,
                  row.short_name, row.category, row.code), dat))
    db_cur.executemany(dedent("""\
        INSERT INTO "emoji" \
        ("id", "emoji", "short_name", "category", "code") \
        VALUES (?, ?, ?, ?, ?);"""),
                       tuples)


def build_sqlite_db(
        db_file, freq, conv, inputs, syls, symbol_file, emoji_file):
    print("Building database, please wait...", end='')
    con = sqlite3.connect(db_file)
    con.set_progress_handler(show_progress, 30)
    cur = con.cursor()
    cur.executescript(build_sql(freq, conv, inputs, syls))
    # cur.executescript(init_db_sql())
    # cur.executescript(frequency_sql(freq))
    # cur.executescript(conversion_sql(conv))
    # cur.executescript(syls_sql(syls))

    if symbol_file is not None:
        build_symbols_table(cur, symbol_file)

    if emoji_file is not None:
        build_emoji_table(cur, emoji_file)

    cur.executescript('VACUUM;')

##############################################################################
#
# __main__
#
##############################################################################


parser = argparse.ArgumentParser(
    description="""Build an SQLite database for the Khiin IME

- The frequencies CSV must have columns: input, freq, chhan_id
- The conversions CSV must have columns: input, output, weight
- The syllable list TXT file is optional, and should include
  one syllable per line (without tones)

All `input` columns are automatically normalized into lower case,
space-separated syllables.

All data files are automatically deduplicated.

""", formatter_class=argparse.RawDescriptionHelpFormatter)
parser.add_argument('-f', "--frequencies", metavar='FILE',
                    required=True, help='the frequencies list CSV file name')

parser.add_argument('-c', "--conversions", metavar='FILE',
                    required=True, help='the conversion CSV file name')

parser.add_argument(
    '-s', "--syllables", metavar='FILE', required=False,
    help='additional list of syllables to include; \
a plain text file with one syllable per line')

parser.add_argument(
    '-t', "--tones", action='store_true',
    help='automatically add all tones to all additional syllables')

parser.add_argument('-o', '--output', metavar='FILE',
                    required=True, help='the output file name')

parser.add_argument('-x', "--exclude-zeros", action='store_true',
                    help='exclude zero-frequency items from the frequency CSV')

parser.add_argument(
    '-j', "--hanji-first", action='store_true',
    help='Automatically weight any Hanji to 1000 and Loji to 900')

parser.add_argument('-d', '--db', required=False,
                    help='Build an SQlite database directly')

parser.add_argument('-y', '--symbols', metavar='FILE',
                    help='Include a tab-delimited symbols csv table')

parser.add_argument('-e', '--emoji', metavar='FILE',
                    help='Include the emoji csv file as a table')

if __name__ == "__main__":
    args = parser.parse_args()

    freq_file = args.frequencies
    conv_file = args.conversions
    syls_file = args.syllables
    sql_file = args.output
    exclude_zeros = args.exclude_zeros
    hanji_first = args.hanji_first
    db_file = args.db
    symbol_file = args.symbols
    emoji_file = args.emoji

    freq_csv = csv.parse_freq_csv(freq_file, exclude_zeros)
    conv_csv = csv.parse_conv_csv(conv_file, hanji_first)
    syls_csv = csv.parse_syls_txt(syls_file)

    if args.tones:
        for syl in syls_csv:
            for sylt in lomaji.add_all_tones(syl):
                freq_csv.append(Frequency(
                    {'input': sylt, 'freq': 0, 'chhan_id': 99999}))
                conv_csv.append(Conversion(
                    {'input': sylt, 'output': sylt,
                     'hint': '', 'weight': 900, 'color': ''}))

    freq_dat = Frequency.dedupe(freq_csv)
    conv_dat = Conversion.dedupe(conv_csv)

    # syls_dat = get_extra_syllables(syls_dat, freq_dat, conv_dat)
    (freq_dat, conv_dat) = find_common_inputs(freq_dat, conv_dat)
    input_dat = get_input_sequences(freq_dat)

    sql = build_sql(freq_dat, conv_dat, input_dat, syls_csv)
    write_sql(sql_file, sql)

    if db_file:
        build_sqlite_db(db_file, freq_dat, conv_dat, input_dat,
                        syls_csv, symbol_file, emoji_file)

    print(f"""Output written to {sql_file}:
 - {len(freq_dat)} inputs ("frequency" table)
 - {len(conv_dat)} tokens ("conversion" table)
 - {len(syls_csv)} syllables ("syllables" table)""")
