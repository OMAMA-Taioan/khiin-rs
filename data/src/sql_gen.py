import argparse
import csv
from functools import cmp_to_key
import itertools
import locale
from operator import itemgetter
from pathlib import Path
import sqlite3
import sys
import re
import unicodedata
locale.setlocale(locale.LC_ALL, '')

from lomaji import to_input_sequences

##############################################################################
#
# Utilities
#
##############################################################################

def has_hanji(string):
    return any(ord(c) > 0x2e80 for c in string)

def normalize_loji(input, strip_tones=False):
    input = unicodedata.normalize('NFD', input)
    input = input.replace('-', ' ').replace('·', '').lower()

    if strip_tones is True:
        input = re.sub(r'[\u0300-\u030d]', '', input)

    return unicodedata.normalize('NFC', input)

spinner = itertools.cycle(['-', '/', '|', '\\'])
def show_progress():
    global spinner
    sys.stdout.write(next(spinner))
    sys.stdout.flush()
    sys.stdout.write('\b')
    return 0

def compare(x, y):
    return (x > y) - (x < y)

def freq_sort(left, right):
    cmp = -compare(left['freq'], right['freq'])
    return cmp if cmp != 0 else compare(left['chhan_id'], right['chhan_id'])

def conv_sort(left, right):
    cmp = compare(locale.strxfrm(left['input']), locale.strxfrm(right['input']))
    return cmp if cmp != 0 else -compare(left['weight'], right['weight'])

freq_sort_key = cmp_to_key(freq_sort)
conv_sort_key = cmp_to_key(conv_sort)
syls_sort_key = locale.strxfrm

##############################################################################
#
# CSV / TXT data parsing
#
##############################################################################

def parse_freq_csv(csv_file, exclude_zeros=False):
    data = []
    with open(csv_file) as f:
        reader = csv.DictReader(f, skipinitialspace=True)
        data = list(reader)
    for x in data:
        x['input'] = normalize_loji(x['input'])
        x['freq'] = int(x['freq'])
        x['chhan_id'] = int(x['chhan_id'])
    if exclude_zeros is True:
        data = [x for x in data if x['freq'] != 0]
    return sorted(data, key=freq_sort_key)

def parse_conv_csv(csv_file, sort_hanji_first):
    data = []
    with open(csv_file) as f:
        reader = csv.DictReader(f, skipinitialspace=True)
        data = list(reader)
    for x in data:
        x['input'] = normalize_loji(x['input'])
        x['weight'] = int(x['weight'])
        if sort_hanji_first is True:
            if has_hanji(x['output']):
                x['weight'] = 1000
            else:
                x['weight'] = 900
    return sorted(data, key=conv_sort_key)

def parse_syls_txt(txt_file):
    data = []
    if not txt_file:
        return data
    with open(txt_file) as f:
        data = [line.rstrip() for line in f]
    return sorted(data, key=syls_sort_key)

##############################################################################
#
# Data validation and collection
#
##############################################################################

def dedupe_conversions(conv_dat):
    seen = set()
    ret = []
    for x in conv_dat:
        if (x['input'], x['output']) not in seen:
            seen.add((x['input'], x['output']))
            ret.append(x)
    return ret

def dedupe_frequencies(freq_dat):
    seen = set()
    ret = []
    for x in freq_dat:
        if x['input'] not in seen:
            seen.add(x['input'])
            ret.append(x)
    return ret

def get_tone_position(syl):
    found = re.search(r"o[ae][a-z]", syl)
    if found is not None:
        return found.start() + 1
    for x in ['o', 'a', 'e', 'u', 'i', 'ṳ', 'n', 'm']:
        found = syl.find(x)
        if found > -1:
            return found
    return -1

def add_all_tones(syl):
    ret = [syl]
    if re.search(r"[ptkh]ⁿ?$", syl):
        tones = ['\u030d']
    else:
        tones = ['\u0301', '\u0300', '\u0302', '\u0304', '\u0306']
    pos = get_tone_position(syl) + 1
    for tone in tones:
        syl_tone = syl[:pos] + tone + syl[pos:]
        syl_tone = unicodedata.normalize('NFC', syl_tone)
        ret.append(syl_tone)
    return ret

def dedupe_syllables(syl_dat):
    return sorted(list(set(syl_dat)), key=syls_sort_key)

def get_unique(dataset, prop):
    ret = set()
    for each in dataset:
        ret.add(each[prop])
    return list(ret)

def find_common_inputs(freq, conv):
    unique_freq_inputs = get_unique(freq, 'input')
    unique_conv_inputs = get_unique(conv, 'input')

    freq_has_conv = []
    conv_has_freq = []

    for word in freq:
        if word['input'] in unique_conv_inputs:
            freq_has_conv.append(word)
    
    for word in conv:
        if word['input'] in unique_freq_inputs:
            conv_has_freq.append(word)

    freq = sorted(freq_has_conv, key=freq_sort_key)
    conv = sorted(conv_has_freq, key=conv_sort_key)
    return [freq, conv]

def get_input_sequences(freq):
    input_seqs = []
    for row in freq:
        row_seqs = to_input_sequences(row['input'])
        for pair in row_seqs:
            input_seqs.append({
                'input': row['input'],
                'numeric': pair[0],
                'telex': pair[1]
            })
    return input_seqs


def get_extra_syllables(syls, freq, conv):
    ret = set(syls)
    for x in freq:
        for syl in x["input"].split(' '):
            ret.add(normalize_loji(syl, True))
    for x in conv:
        for syl in x["input"].split(' '):
            ret.add(normalize_loji(syl, True))
    return sorted(list(ret), key=syls_sort_key)

##############################################################################
#
# SQL builder functions
#
##############################################################################

def init_db_sql():
    return """DROP TABLE IF EXISTS "metadata";
DROP TABLE IF EXISTS "conversions";
DROP TABLE IF EXISTS "frequency";
DROP TABLE IF EXISTS "input_numeric";
DROP TABLE IF EXISTS "input_telex";
DROP TABLE IF EXISTS "syllables";
DROP INDEX IF EXISTS "unigram_freq_gram_idx";
DROP TABLE IF EXISTS "unigram_freq";
DROP INDEX IF EXISTS "bigram_freq_gram_index";
DROP TABLE IF EXISTS "bigram_freq";

CREATE TABLE IF NOT EXISTS "metadata" (
    "key"	TEXT,
    "value"	INTEGER
);

CREATE TABLE IF NOT EXISTS "frequency" (
	"id"        INTEGER PRIMARY KEY,
    "input"     TEXT NOT NULL,
	"freq"      INTEGER,
	"chhan_id"  INTEGER,
	UNIQUE("input")
);

CREATE TABLE IF NOT EXISTS "conversions" (
    "input_id"     INTEGER,
    "output"       TEXT NOT NULL,
    "weight"       INTEGER,
    "category"     INTEGER,
    "annotation"   TEXT,
    UNIQUE("input_id","output"),
    FOREIGN KEY("input_id") REFERENCES "frequency"("id")
);

CREATE TABLE IF NOT EXISTS "input_numeric" (
    "input_id"      INTEGER,
    "key_sequence"  TEXT NOT NULL,
    UNIQUE("input_id","key_sequence"),
    FOREIGN KEY("input_id") REFERENCES "frequency"("id")
);

CREATE TABLE IF NOT EXISTS "input_telex" (
    "input_id"      INTEGER,
    "key_sequence"  TEXT NOT NULL,
    UNIQUE("input_id","key_sequence"),
    FOREIGN KEY("input_id") REFERENCES "frequency"("id")
);

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

CREATE INDEX "input_numeric_covering_index" ON "input_numeric" (
    "key_sequence",
    "input_id"
);

CREATE INDEX "input_telex_covering_index" ON "input_telex" (
    "key_sequence",
    "input_id"
);

CREATE INDEX "unigram_gram_index" ON "unigram_freq" (
    "gram"
);

CREATE INDEX "bigram_gram_index" ON "bigram_freq" (
    "rgram",
    "lgram"
);

DROP VIEW IF EXISTS "lookup_numeric";
CREATE VIEW "lookup_numeric" (
    key_sequence,
    input,
    input_id,
    output,
    weight,
    category,
    annotation
) as SELECT
    n.key_sequence,
    f.input,
    n.input_id,
    c.output,
    c.weight,
    c.category,
    c.annotation
FROM input_numeric AS n
JOIN frequency AS f ON f.id = n.input_id
JOIN conversions AS c ON f.id = c.input_id;

DROP VIEW IF EXISTS "lookup_telex";
CREATE VIEW "lookup_telex" (
    key_sequence,
    input,
    input_id,
    output,
    weight,
    category,
    annotation
) as SELECT
    t.key_sequence,
    f.input,
    t.input_id,
    c.output,
    c.weight,
    c.category,
    c.annotation
FROM input_telex AS t
JOIN frequency AS f ON f.id = t.input_id
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
"""

def symbol_row_sql(row):
    return f'("row[]"'

def frequency_row_sql(row):
    return f'("{row["input"]}", {row["freq"]}, {row["chhan_id"]})'

def frequency_sql(data):
    sql = 'INSERT INTO "frequency" ("input", "freq", "chhan_id") VALUES\n'
    values = [frequency_row_sql(x) for x in data]
    sql += ',\n'.join(values) + ';\n'
    return sql

def conversion_row_sql(row):
    return f'INSERT INTO "conversions" ("input_id", "output", "weight") SELECT "id", "{row["output"]}", {row["weight"]} FROM "frequency" WHERE "input"="{row["input"]}";'

def conversion_sql(data):
    values = [conversion_row_sql(row) for row in data]
    sql = '\n'.join(values) + '\n'
    return sql

def telex_input_row_sql(row):
    return f'INSERT INTO "input_telex" ("input_id", "key_sequence") SELECT "id", "{row["telex"]}" FROM "frequency" WHERE "input"="{row["input"]}";'

def numeric_input_row_sql(row):
    return f'INSERT INTO "input_numeric" ("input_id", "key_sequence") SELECT "id", "{row["numeric"]}" FROM "frequency" WHERE "input"="{row["input"]}";'

def input_sql(data):
    numeric = [numeric_input_row_sql(row) for row in data]
    telex = [telex_input_row_sql(row) for row in data]
    sql = '\n'.join(numeric) + '\n' + '\n'.join(telex) + '\n'
    return sql

def syls_sql(data):
    sql = 'INSERT INTO "syllables" ("input") VALUES\n'
    values = ',\n'.join([f'("{x}")' for x in data])
    sql += values + ';\n'
    return sql

def build_sql(freq, conv, inputs, syls):
    sql = """
PRAGMA journal_mode = OFF;
PRAGMA cache_size = 7500000;
PRAGMA synchronous = OFF;
PRAGMA temp_store = 2;
BEGIN TRANSACTION;
    """
    sql += init_db_sql()
    sql += frequency_sql(freq)
    sql += conversion_sql(conv)
    sql += input_sql(inputs)
    sql += syls_sql(syls) if (len(syls) > 0) else ""
    sql += """
COMMIT;
PRAGMA journal_mode = WAL;
PRAGMA cache_size = -2000;
PRAGMA synchronous = NORMAL;
PRAGMA temp_store = 0;
    """
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
    db_cur.executescript("""
    DROP TABLE IF EXISTS "symbols";
    CREATE TABLE "symbols" (
        "id"           INTEGER PRIMARY KEY,
        "input"        TEXT NOT NULL,
        "output"       TEXT NOT NULL,
        "category"     INTEGER,
        "annotation"   TEXT
    );
    """)
    dat = []
    with open(symbol_tsv, 'r') as f:
        rows = csv.DictReader(f, delimiter='\t')
        dat = [(x['input'], x['output'], x['category']) for x in rows]
    db_cur.executemany('INSERT INTO "symbols" ("input", "output", "category") VALUES (?, ?, ?);', dat)

def build_emoji_table(db_cur, emoji_csv):
    db_cur.executescript("""
    DROP TABLE IF EXISTS "emoji";
    CREATE TABLE "emoji" (
        id INTEGER PRIMARY KEY,
        emoji TEXT NOT NULL,
        short_name TEXT NOT NULL,
        category INTEGER NOT NULL,
        code TEXT NOT NULL
    );
    """)
    dat = []
    with open(emoji_csv, 'r') as f:
        rows = csv.DictReader(f)
        filter(lambda x: x['recent'] == 1, rows)
        dat = [(x['id'], x['emoji'], x['short_name'], x['category'],  x['code']) for x in rows]
    db_cur.executemany('INSERT INTO "emoji" ("id", "emoji", "short_name", "category", "code") VALUES (?, ?, ?, ?, ?);', dat)

def build_sqlite_db(db_file, freq, conv, inputs, syls, symbol_file, emoji_file):
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

parser = argparse.ArgumentParser(description="""Build an SQLite database for the Khiin IME

- The frequencies CSV must have columns: input, freq, chhan_id
- The conversions CSV must have columns: input, output, weight
- The syllable list TXT file is optional, and should include
  one syllable per line (without tones)

All `input` columns are automatically normalized into lower case,
space-separated syllables.

All data files are automatically deduplicated.

""", formatter_class=argparse.RawDescriptionHelpFormatter)
parser.add_argument('-f', "--frequencies", metavar='FILE', required=True, help='the frequencies list CSV file name')
parser.add_argument('-c', "--conversions", metavar='FILE', required=True, help='the conversion CSV file name')
parser.add_argument('-s', "--syllables", metavar='FILE', required=False, help='additional list of syllables to include; a plain text file with one syllable per line')
parser.add_argument('-t', "--tones", action='store_true', help='automatically add all tones to all additional syllables')
parser.add_argument('-o', '--output', metavar='FILE', required=True, help='the output file name')
parser.add_argument('-x', "--exclude-zeros", action='store_true', help='exclude zero-frequency items from the frequency CSV')
parser.add_argument('-j', "--hanji-first", action='store_true', help='Automatically weight any Hanji to 1000 and Loji to 900')
parser.add_argument('-d', '--db', required=False, help='Build an SQlite database directly')
parser.add_argument('-y', '--symbols', metavar='FILE', help='Include a tab-delimited symbols csv table')
parser.add_argument('-e', '--emoji', metavar='FILE', help='Include the emoji csv file as a table')

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

    freq_csv = parse_freq_csv(freq_file, exclude_zeros)
    conv_csv = parse_conv_csv(conv_file, hanji_first)
    syls_dat = dedupe_syllables(parse_syls_txt(syls_file))
    
    if args.tones:
        for syl in syls_dat:
            for sylt in add_all_tones(syl):
                freq_csv.append({ 'input': sylt, 'freq': 0, 'chhan_id': 99999 })
                conv_csv.append({ 'input': sylt, 'output': sylt, 'hint': '', 'weight': 900, 'color': '' })

    freq_dat = dedupe_frequencies(freq_csv)
    conv_dat = dedupe_conversions(conv_csv)

    # syls_dat = get_extra_syllables(syls_dat, freq_dat, conv_dat)
    [freq_dat, conv_dat] = find_common_inputs(freq_dat, conv_dat)
    input_dat = get_input_sequences(freq_dat)

    sql = build_sql(freq_dat, conv_dat, input_dat, syls_dat)
    write_sql(sql_file, sql)

    if db_file:
        build_sqlite_db(db_file, freq_dat, conv_dat, input_dat, syls_dat, symbol_file, emoji_file)

    print(f"""Output written to {sql_file}:
 - {len(freq_dat)} inputs ("frequency" table)
 - {len(conv_dat)} tokens ("conversion" table)
 - {len(syls_dat)} syllables ("syllables" table)""")
