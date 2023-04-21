import re
from textwrap import dedent
from typing import List

import lomaji

T_FREQ = "frequency"
T_CONV = "conversions"
T_KEYSEQ = "key_sequences"
T_METADATA = "metadata"
T_SYL = "syllables"
T_SYM = "symbols"
T_EMO = "emoji"
T_UGRAM = "unigram_freq"
T_BGRAM = "bigram_freq"
V_LOOKUP = "conversion_lookups"
V_GRAMS = "ngrams"


def collapse(text: str) -> str:
    return re.sub(r'\s+', ' ', text)


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
        return dedent(f"""\
            create table if not exists "{T_FREQ}" (
                "id"        integer primary key,
                "input"     text not null,
                "freq"      integer,
                "chhan_id"  integer,
                unique("input")
            );""")

    @staticmethod
    def insert_statement(rows: List['Frequency']):
        sql = dedent(f"""insert into "{T_FREQ}" \
                     ("input", "freq", "chhan_id") values\n""")
        values = [row.to_tuple() for row in rows]
        sql += ',\n'.join(values) + ';\n'
        return sql

    def to_tuple(self):
        return f"('{self.input}', {self.freq}, {self.chhan_id})"


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
        return dedent(f"""
            create table if not exists "{T_CONV}" (
                "input_id"     integer,
                "output"       text not null,
                "weight"       integer,
                "category"     integer,
                "annotation"   text,
                unique("input_id", "output"),
                foreign key("input_id") references "{T_FREQ}"("id")
            );""")

    @staticmethod
    def insert_statement(rows: List['Conversion']):
        values = [row.insert_sql() for row in rows]
        sql = '\n'.join(values) + '\n'
        return sql

    def insert_sql(self):
        return collapse(f"""\
            insert into "{T_CONV}" \
            ("input_id", "output", "weight") \
            select "f"."id", '{self.output}', {self.weight} \
            from "{T_FREQ}" as "f" \
            where f.input='{self.input}';""")


class SqlInputSeq:
    def __init__(self, input: str, numeric: str, telex: str, n_syls: int,
                 fuzzy_tone: bool, p: float) -> None:
        self.input = input
        self.numeric = numeric
        self.telex = telex
        self.n_syls = n_syls
        self.fuzzy_tone = fuzzy_tone
        self.p = p

    @staticmethod
    def create_statement():
        return dedent(f"""
            create table if not exists "{T_KEYSEQ}" (
                "input_id"      integer,
                "numeric"       text not null,
                "telex"         text not null,
                "n_syls"        integer,
                "fuzzy_tone"    boolean,
                "p"             real,
                unique("input_id", "numeric"),
                foreign key("input_id") references "{T_FREQ}"("id")
            );""")

    def insert_row(self):
        return collapse(f"""\
        insert into "{T_KEYSEQ}" \
        ("input_id", "numeric", "telex", "n_syls", "fuzzy_tone", "p") \
        select \
        "f"."id", \
        '{self.numeric}', \
        '{self.telex}', \
        {self.n_syls}, \
        {self.fuzzy_tone}, \
        {self.p} \
        from "{T_FREQ}" as "f" \
        where "f"."input"='{self.input}';""")

    def insert_statement(rows: List['SqlInputSeq']) -> str:
        insert_list = [row.insert_row() for row in rows]
        sql = '\n'.join(insert_list) + '\n'
        return sql


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
