from textwrap import dedent
from typing import List

import lomaji


class Frequency:
    def __init__(self, row) -> None:
        self.input = lomaji.normalize_loji(row['input'])
        self.freq = int(row['freq'])
        self.chhan_id = int(row['chhan_id'])

    @staticmethod
    def dedupe(items: List['Frequency']) -> List['Frequency']:
        seen = set()
        ret = []
        for each in items:
            if each.input not in seen:
                seen.add(each.input)
                ret.append(each)
        return ret

    @staticmethod
    def create_statement():
        return dedent("""\
            CREATE TABLE IF NOT EXISTS "frequency" (
                "id"        INTEGER PRIMARY KEY,
                "input"     TEXT NOT NULL,
                "freq"      INTEGER,
                "chhan_id"  INTEGER,
                UNIQUE("input")
            );""")

    @staticmethod
    def insert_statement(rows: List['Frequency']):
        sql = 'INSERT INTO "frequency" ("input", "freq", "chhan_id") VALUES\n'
        values = [row.to_tuple() for row in rows]
        sql += ',\n'.join(values) + ';\n'
        return sql

    def to_tuple(self):
        return f'("{self.input}", "{self.freq}", "{self.chhan_id}")'


class Conversion:
    def __init__(self, row, fixed_weights=False) -> None:
        self.input = lomaji.normalize_loji(row['input'])
        self.output = row['output']
        self.category = row['color']
        self.annotation = row['hint']

        if fixed_weights is True:
            if lomaji.has_hanji(self.output):
                self.weight = 1000
            else:
                self.weight = 900
        else:
            self.weight = int(row['weight'])

    @staticmethod
    def dedupe(items: List['Conversion']) -> List['Conversion']:
        seen = set()
        ret = []
        for each in items:
            if (each.input, each.output) not in seen:
                seen.add((each.input, each.output))
                ret.append(each)
        return ret

    @staticmethod
    def create_statement():
        return dedent("""
            CREATE TABLE IF NOT EXISTS "conversions" (
                "input_id"     INTEGER,
                "output"       TEXT NOT NULL,
                "weight"       INTEGER,
                "category"     INTEGER,
                "annotation"   TEXT,
                UNIQUE("input_id","output"),
                FOREIGN KEY("input_id") REFERENCES "frequency"("id")
            );""")

    @staticmethod
    def insert_statement(rows: List['Conversion']):
        values = [row.insert_sql() for row in rows]
        sql = '\n'.join(values) + '\n'
        return sql

    def insert_sql(self):
        return dedent(f"""\
            INSERT INTO "conversions" ("input_id", "output", "weight") \
            SELECT "id", "{self.output}", {self.weight} \
            FROM "frequency" WHERE "input"="{self.input}";""")


class SqlInputSeq:
    def __init__(self, input: str, numeric: str, telex: str, n_syls: int,
                 p: float) -> None:
        self.input = input
        self.numeric = numeric
        self.telex = telex
        self.n_syls = n_syls
        self.p = p

    @staticmethod
    def create_statement():
        return dedent("""
            CREATE TABLE IF NOT EXISTS "input_sequences" (
                "input_id"      INTEGER,
                "numeric"       TEXT NOT NULL,
                "telex"         TEXT NOT NULL,
                "n_syls"        INTEGER,
                "p"             REAL,
                UNIQUE("input_id","numeric"),
                FOREIGN KEY("input_id") REFERENCES "frequency"("id")
            );""")

    def insert_row(self):
        return dedent(f"""\
            INSERT INTO "input_sequences"
                ("input_id", "numeric", "telex", "n_syls", "p") \
            SELECT \
                "id",\
                "{self.numeric}",\
                "{self.telex}",\
                "{self.n_syls}",\
                "{self.p}"\
            FROM "frequency"\
            WHERE "input"="{self.input}";""")


class Symbols:
    def __init__(self, row) -> None:
        self.input = row['input']
        self.output = row['output']
        self.category = row['category']


class Emoji:
    def __init__(self, row) -> None:
        self.id = row['id']
        self.emoji = row['emoji']
        self.short_name = row['short_name']
        self.category = row['category']
        self.code = row['code']
