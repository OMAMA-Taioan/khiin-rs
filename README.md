# Khíín Taiwanese IME

This is an in-progress Rust rewrite of the first (C++) version of Khíín.

## Installation

## Usage

## Help

---

# Development

```
khiin-rs/
├── android/
│   ├── app         # Jetpack Compose Android app
│   └── rust        # JNI glue library for khiin
├── cli             # Terminal application (for developers)
├── data            # SQLite db generator
├── khiin           # Cross-platform engine
├── protos          # Protobuf definitions
├── resources/
│   └── khiin.db    # Generated db file
├── swift           # iOS and macOS applications
├── windows/
│   ├── ime/        # TSF library
│   ├── res/        # Windows specific resources
│   └── settings/   # Settings app
└── Makefile.toml   # Cargo build tasks
```

## Database

```
khiin-rs/
├── data/
│   ├── data/
│   │   ├── conversions_all.csv
│   │   └── frequency.csv
│   └── src/
│       ├── sql_gen.py
│       └── tables.py
```
        
You must build `khiin.db` before most things will work. The build command is
included in `launch.json`. If you run the command directly from VS Code, the
resulting `khiin.db` database file will be placed into the `resources`
directory. The rust libraries copy the file from this directory into the
`target` directory for use during testing and debugging.

- `frequency.csv` contains the romanized wordlist with a rough frequency count
  for each item based on the available corpus.
- `conversions_all.csv` contains the possible outputs (both romanized or hanji)
  for a given word, plus additional information.

The python scripts convert the romanized wordlist into ASCII key sequences,
accounting for users who decide to type tones or not, and builds tables for
numeric and telex input sequences, as well as a table listing the probability of
each word based on the frequency counts.

The database is continually updated with user data during use, to improve
candidate prediction based on a simple N-gram model that currently uses 1-gram
and 2-gram frequencies. In the future this may be extended to other precition
algorithms for better results.

In addition to `khiin.db`, users may provide an additional custom dictionary
file, which is simply a text file listing rows of space-delimited `input output`
options to display as candidates. (Everything after the first space is taken as
the output.) These candidates are displayed in addition to the default database.

At present, data is not shared at all, and is strictly used within the
application itself. In future we would like to add an option to sync user's data
across devices, and an option to allow users to share their (anonymized) data
with us for improving our corpus.

## Khiin (Engine)

The engine maintains a stateful buffer during each input session with a client
application. The buffer can contain various types of items depending on whether
the input sequence has matches in the conversion database, the user's own custom
database, or full/half-width punctuation, etc.

Candidate search is a two-step process. We first use the word list probabilities
to segment the user's input sequence (if they don't segment it themselves while
typing), and then search for each segment in the conversion table, using the
unigram and bigram records to sort the resulting options.

## Protobuf

The engine and client applications communicate using protocol buffers. The
engine exposes a single function endpoint:

```rust
send_command_bytes(bytes: &[u8]) -> Vec<u8>
```

The input/output bytes are both a serialized `khiin_protos::Command` protocol
buffer, which contains both a `Request` and a `Response` message. Clients should
tag each `Request` with an id, so that the client can associate the correct
`Response`.

## Windows App

The Windows IME is mostly complete, although it is still missing a few key
features for release. The app includes the IME itself, as well as a Settings
application that allows the user to configure the IME, and a basic WiX installer
(TODO).

See the [README](windows/README.md) for more details.

---

## Android App

The Android IME is currently in progress / unstable. It is a modern Jetpack
Compose app written in Kotlin. The `android/rust` folder contains a small
wrapper around `khiin` that communicates with the Android app via JNI.

See the [README](android/README.md) for more details.

---

## iOS & macOS Apps

The iOS app is currently in progress / unstable. Basic setup between the Khiin
engine and the app is complete, so the remaining work is mainly to build out the
UI. Development has not yet started on the macOS app.

See the [README](swift/README.md) for more details.

---

## Development CLI App

This is a very basic terminal application intended for developers or database
maintainers to quickly work on the engine and preview changes without needing to
load up a full application. The terminal application has been tested to work on
any platform, including Windows.

This tool demonstrates all of the available features of the engine, which can be
used in the various client applications. Follow the quickstart guide below for
setting up the CLI app independently of any other client applications.

### Quickstart

- `python3` must be installed and available in the system path
- All commands should be run from the root `khiin-rs` directory

Install rust:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Reload your terminal to source the new files. Then install `cargo-make`:

```
cargo install --force cargo-make
```

Clone this repo and build it:

```
git clone https://github.com/aiongg/khiin-rs.git
cd khiin-rs
cargo make
```

Run the terminal application:

```
./target/debug/khiin_cli
```

To rebuild the database after an update, run:

```
cargo make db-rebuild
```
