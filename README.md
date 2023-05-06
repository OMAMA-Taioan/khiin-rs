# Khíín Taiwanese IME

![CI Build & Test](https://github.com/aiongg/khiin-rs/actions/workflows/ci.yml/badge.svg?branch=master)

**Khíín** is a cross-platform input method for typing Taiwanese. We aim to
support all major platforms, including Windows, Android, macOS, iOS, Linux, and
the web. Our goal is simple: to provide an excellent Taiwanese typing
experience.

If you are interested in contributing, please open an issue!

- [Khíín Taiwanese IME](#khíín-taiwanese-ime)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Help](#help)
- [Development](#development)
  - [Database](#database)
  - [Khiin (Engine)](#khiin-engine)
  - [App (Settings \& Guide)](#app-settings--guide)
  - [Protobuf](#protobuf)
  - [Windows App](#windows-app)
  - [Android App](#android-app)
  - [iOS \& macOS Apps](#ios--macos-apps)
  - [Development CLI App](#development-cli-app)
    - [Quickstart](#quickstart)


## Installation

TODO

## Usage

TODO

## Help

TODO

---

# Development

```
khiin-rs/
├── android/
│   ├── app         # Jetpack Compose Android app
│   └── rust        # JNI glue library for khiin
├── app/            # Settings & help app
│   ├── frontned    # Svelte frontend
│   ├── settings    # Settings manager (Khiin.toml)
│   └── src-tauri   # Tauri backend
├── cli/            # Terminal application (for developers)
├── data/           # CSV databases (Provided by Tâi Jī Siā)
├── ji/             # Taiwanese script handling library
├── khiin/          # Cross-platform engine library
│   └── dbgen/      # CLI tool to generate the DB
├── protos/         # Protobuf definitions
├── resources/
│   └── khiin.db    # Generated db file
├── swift/          # iOS and macOS applications
├── windows/
│   ├── ime/        # TSF library
│   ├── res/        # Windows specific resources
│   └── service/    # Engine manager service
└── Makefile.toml   # Cargo build tasks
```

## Database

```
khiin-rs/
├── data/
│   ├── data/
│   │   ├── conversions_all.csv
│   │   └── frequency.csv
│   └── Cargo.toml
│
│
```
        
The engine library embeds the CSVs and produces the database at first run. For inspection of the database, use the CLI tool included in the engine crate. See below for details.

- `frequency.csv` contains the romanized wordlist with a rough frequency count
  for each item based on the available corpus.
- `conversions_all.csv` contains the possible outputs (both romanized or hanji)
  for a given word, plus additional information.

The database generator converts the romanized wordlist into ASCII key sequences,
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

## App (Settings & Guide)

The app is a [Tauri](https://tauri.app/) & [Svelte](https://svelte.dev/)
cross-platform app built in Rust & TypeScript. The goal here is to provide a
consistent UI for managing settings on all desktop platforms, and to provide
instructions for using the IME or other useful materials.

See the [README](app/README.md) for more details.

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
application that allows the user to configure the IME, and a basic WiX installer.

See the [README](windows/README.md) for more details.

## Android App

The Android IME is currently in progress / unstable. It is a modern Jetpack
Compose app written in Kotlin. The `android/rust` folder contains a small
wrapper around `khiin` that communicates with the Android app via JNI.

See the [README](android/README.md) for more details.

## iOS & macOS Apps

The iOS and macOS apps are currently in progress / unstable. Basic setup between
the Khiin engine and the apps is complete, so the remaining work is mainly to
build out the UI and hook up all of the engine wiring.

See the [README](swift/README.md) for more details.

## Development CLI App

This is a very basic terminal application intended for developers or database
maintainers to quickly work on the engine and preview changes without needing to
load up a full application. The terminal application has been tested to work on
any platform, including Windows.

This tool demonstrates all of the available features of the engine, which can be
used in the various client applications. Follow the quickstart guide below for
setting up the CLI app independently of any other client applications.

### Quickstart


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
cargo make build-cli
```

Run the terminal application:

```
./target/debug/khiin_cli
```

To rebuild the database after an update, run:

```
cargo make rebuild-db
```
