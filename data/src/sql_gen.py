from textwrap import dedent
import csv_parsers as csv
import lomaji
import os
import argparse
import itertools
import locale
import sqlite3
import sys
from typing import List, Tuple

from tables import *

locale.setlocale(locale.LC_ALL, '')

spinner = itertools.cycle(['-', '/', '|', '\\'])


def show_progress():
    global spinner
    sys.stdout.write(next(spinner))
    sys.stdout.flush()
    sys.stdout.write('\b')
    return 0


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
        for i, (numeric, telex, n_syls) in enumerate(row_seqs):
            fuzzy_tone = i > 0 and n_syls == 1
            input_seqs.append(SqlInputSeq(
                row.input, numeric, telex, n_syls, fuzzy_tone, p=row.freq / total_count
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


SQL_HEADER = f"""\
pragma journal_mode = OFF;
pragma cache_size = 7500000;
pragma synchronous = OFF;
pragma temp_store = 2;
begin transaction;
"""

SQL_FOOTER = """\
commit;
pragma journal_mode = WAL;
pragma cache_size = -2000;
pragma synchronous = NORMAL;
pragma temp_store = 0;
"""

INIT_DB_SQL = f"""
drop table if exists "{T_METADATA}";
drop table if exists "{T_CONV}";
drop table if exists "{T_FREQ}";
drop table if exists "{T_KEYSEQ}";
drop table if exists "{T_SYL}";
drop table if exists "{T_UGRAM}";
drop table if exists "{T_BGRAM}";

create table if not exists "{T_METADATA}" (
    "key"	text,
    "value"	integer
);

{Frequency.create_statement()}
{Conversion.create_statement()}
{SqlInputSeq.create_statement()}

create table if not exists "{T_SYL}" (
    "input"   text not null unique
);

create table if not exists "{T_UGRAM}" (
    "gram"	text not null unique,
    "n"	integer not null
);

create table if not exists "{T_BGRAM}" (
    "lgram"	text,
    "rgram"	text,
    "n"	integer not null,
    unique("lgram","rgram")
);

create index "conversions_input_id_covering_index" on "{T_CONV}" (
    "input_id",
    "output",
    "weight",
    "category",
    "annotation"
);

create index "input_numeric_covering_index" on "{T_KEYSEQ}" (
    "numeric",
    "input_id"
);

create index "input_telex_covering_index" on "{T_KEYSEQ}" (
    "telex",
    "input_id"
);

create index "unigram_gram_index" on "{T_UGRAM}" (
    "gram"
);

create index "bigram_gram_index" on "{T_BGRAM}" (
    "rgram",
    "lgram"
);

drop view if exists "{V_LOOKUP}";
create view "{V_LOOKUP}" (
    "numeric",
    "telex",
    "n_syls",
    "input",
    "input_id",
    "output",
    "weight",
    "category",
    "annotation"
) as select
    "n"."numeric",
    "n"."telex",
    "n"."n_syls",
    "f"."input",
    "n"."input_id",
    "c"."output",
    "c"."weight",
    "c"."category",
    "c"."annotation"
from "{T_KEYSEQ}" as "n"
join "{T_FREQ}" as "f" on "f"."id" = "n"."input_id"
join "{T_CONV}" as "c" on "f"."id" = "c"."input_id";

drop view if exists "{V_GRAMS}";
create view "{V_GRAMS}" (
    "lgram",
    "rgram",
    "rgram_count",
    "bigram_count"
) as select
    "b"."lgram",
    "u"."gram",
    "u"."n" as "unigram_count",
    "b"."n" as "bigram_count"
from "{T_UGRAM}" as "u"
left join "{T_BGRAM}" as "b" on "u"."gram" = "b"."rgram";
"""


def syls_sql(data: List[str]):
    sql = f'insert into "{T_SYL}" ("input") values\n'
    values = ',\n'.join([f'("{syl}")' for syl in data])
    sql += values + ';\n'
    return sql


def build_sql(
        freq: List[Frequency],
        conv: List[Conversion],
        inputs: List[SqlInputSeq],
        syls: List[str]):

    sql = SQL_HEADER
    sql += INIT_DB_SQL
    sql += Frequency.insert_statement(freq)
    sql += Conversion.insert_statement(conv)
    sql += SqlInputSeq.insert_statement(inputs)
    sql += syls_sql(syls) if (len(syls) > 0) else ""
    sql += SQL_FOOTER
    return sql


def write_sql(sql_file, sql):
    with open(sql_file, 'w', encoding='utf-8') as f:
        f.write(sql)


def build_symbols_table(db_cur, symbol_tsv):
    db_cur.executescript(dedent(f"""\
        drop table if exists "{T_SYM}";
        create table "{T_SYM}" (
            "id"           integer primary key,
            "input"        text not null,
            "output"       text not null,
            "category"     integer,
            "annotation"   text
        );
        """))
    dat = csv.parse_symbols_tsv(symbol_tsv)
    tuples = list(map(lambda row: (row.input, row.output, row.category), dat))
    db_cur.executemany(dedent(f"""\
        insert into "{T_SYM}" \
        ("input", "output", "category") \
        values (?, ?, ?);"""),
                       tuples)


def build_emoji_table(db_cur, emoji_csv):
    db_cur.executescript(dedent(f"""\
        drop table if exists "{T_EMO}";
        create table "{T_EMO}" (
            "id"           integer primary key,
            "emoji"        text not null,
            "short_name"   text not null,
            "category"     integer not null,
            "code"         text not null
        );
        """))
    dat = csv.parse_emoji_csv(emoji_csv)
    tuples = list(map(lambda row: (row.id, row.emoji,
                  row.short_name, row.category, row.code), dat))
    db_cur.executemany(dedent(f"""\
        insert into "{T_EMO}" \
        ("id", "emoji", "short_name", "category", "code") \
        values (?, ?, ?, ?, ?);"""),
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

def ensure_dir(file):
    dir = os.path.dirname(file)
    if not os.path.exists(dir):
        os.makedirs(dir)

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

    ensure_dir(sql_file)
    ensure_dir(db_file)

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
 - {len(freq_dat)} inputs ("{T_FREQ}" table)
 - {len(conv_dat)} tokens ("{T_CONV}" table)
 - {len(syls_csv)} syllables ("{T_SYL}" table)""")
