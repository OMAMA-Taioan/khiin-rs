# khiin-data

Two `csv` files must be provided.

A frequency `csv` with columns:

- `input`: Lomaji input
- `freq`: Raw frequency count
- `chhan_id`: Lowest ID of any entry in the Chhan with this input

A conversions `csv` with columns:

- `input`: Lomaji input
- `output`: Any text output
- `weight`: To order different `output`s with the same `input`
- `category`: An integer (0 = Default, 1 = Fallback, 2 = Extended)
- `annotation`: Hint text to display during candidate selection

An optional plaintext list of toneless syllables may be provided, with
one syllable per line. All syllables from the `input` columns of
both frequency and conversion files, and this additional syllables
list (if provided) will be included in the final output.

All data inputs are automatically deduplicated according to the
following constraints:

- frequency: UNIQUE(input)
- conversions: UNIQUE(input, output), FOREIGN KEY(input) ON frequency(input)

Run the script `sql_gen.py` to generate the database file. View
detailed instructions with `-h`:

```
python3 src/sql_gen.py -h
```

### Build the full DB:

```
mkdir out
python3 src/sql_gen.py \
    -j \
    -f data/frequency.csv \
    -c data/conversions_all.csv \
    -s data/syllables.txt \
    -y data/symbols.tsv \
    -e data/emoji.csv \
    -o out/khiin_db.sql \
    -d out/khiin.db
```

### Build the test DB:

```
mkdir out
python3 src/sql_gen.py \
    -x -j \
    -f data/frequency.csv \
    -c data/conversions_sample.csv \
    -o out/khiin_test_db.sql \
    -d out/khiin_test.db
```

## Emoji

The emoji table is taken directly from Unicode's [Full Emoji List, v14.0](https://unicode.org/emoji/charts/full-emoji-list.html).

1. Smileys 🙂
2. People & Body 👍
3. Animals & Nature 🐱
4. Food & Drink 🍌
5. Travel & Places 🌍
6. Activities ⚾
7. Objects 🔔
8. Symbols 🚻
9. Flags 🏴‍☠️

## FHL & CIN output

After generating your `khiin.db`, run the script to build FHL and CIN output files. The file `TalmageOverride.db` should be placed in `%APPDATA%\FHL TaigiIME\IMTalmage` on Windows or similar location on Mac. The CIN file can be used with 萊姆中文輸入法 - LIME IME, which is no longer available on the store but APKs are available online.

```
python3 -i out/khiin.db \
        -o out/TalmageOverride.db \
        -c out/khiin.cin
```
