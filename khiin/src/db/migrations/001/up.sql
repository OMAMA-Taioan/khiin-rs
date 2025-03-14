drop table if exists metadata;

drop table if exists conversions;

drop table if exists inputs;

drop table if exists key_sequences;

drop table if exists syllables;

drop table if exists unigrams;

drop table if exists bigrams;

create table
    metadata (key text, value integer);

create table
    inputs (
        "id" integer primary key,
        "input" text not null,
        "corpus_count" integer,
        "chhan_id" integer,
        unique ("input")
    );

create table
    conversions (
        "input_id" integer,
        "output" text not null,
        "weight" integer,
        "annotation" text,
        "khin_ok" integer,
        "khinless_ok" integer,
        "is_hanji" integer,
        unique ("input_id", "output"),
        foreign key ("input_id") references "inputs" ("id")
    );

create table
    key_sequences (
        "input_id" integer not null,
        "key_sequence" text not null,
        "input_type" integer not null,
        "n_syls" integer not null,
        "p" real not null,
        foreign key ("input_id") references "inputs" ("id")
    );

create table
    syllables ("input" text not null unique);

create table
    unigrams (
        "gram" text not null unique,
        "n" integer not null
    );

create table
    bigrams (
        "lgram" text,
        "rgram" text,
        "n" integer not null,
        unique ("lgram", "rgram")
    );

create index conversions_input_id_covering_index on conversions (
    "input_id",
    "output",
    "weight",
    "khin_ok",
    "khinless_ok",
    "annotation"
);

create index input_numeric_covering_index on key_sequences ("numeric", "input_id");

create index input_telex_covering_index on key_sequences ("telex", "input_id");

create index unigram_gram_index on unigrams ("gram");

create index bigram_gram_index on bigrams ("rgram", "lgram");

drop view if exists conversion_lookups;

create view
    conversion_lookups (
        "key_sequence",
        "input_type",
        "n_syls",
        "input",
        "input_id",
        "output",
        "weight",
        "khin_ok",
        "khinless_ok",
        "annotation",
        "is_hanji"
    ) as
select
    "n"."key_sequence",
    "n"."input_type",
    "n"."n_syls",
    "f"."input",
    "n"."input_id",
    "c"."output",
    "c"."weight",
    "c"."khin_ok",
    "c"."khinless_ok",
    "c"."annotation",
    "c"."is_hanji"
from
    key_sequences as "n"
    join inputs as "f" on "f"."id" = "n"."input_id"
    join conversions as "c" on "f"."id" = "c"."input_id";

drop view if exists ngrams;

create view
    ngrams (
        "lgram",
        "rgram",
        "rgram_count",
        "bigram_count"
    ) as
select
    "b"."lgram",
    "u"."gram",
    "u"."n" as "unigram_count",
    "b"."n" as "bigram_count"
from
    unigrams as "u"
    left join bigrams as "b" on "u"."gram" = "b"."rgram";
