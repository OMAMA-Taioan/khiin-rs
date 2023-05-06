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

## Building the DB

The `khiin` library will automatically build the SQLite database using these
CSVs during first run. The SQLite file will be saved into the user's app data
directory. There is also a simple CLI tool for building the database during
development.

To build the database using all default options:

```bash
cargo make build-db

# Or, after the first build:
cargo make rebuild-db
```

This will output the database into the `resources` folder for inspection. For
more options, you can build the CLI tool directly, and run it to see the options:

```bash
cargo make build-db-cli
./target/debug/khiin_db_cli -h
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

Note: Not yet available with the rust dbgen tool. If you need it, revert back to [4e79459](https://github.com/aiongg/khiin-rs/tree/4e79459e4cf595580d1579dc0d87a2455f1a1b78/data)
