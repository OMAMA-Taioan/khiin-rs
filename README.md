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

## Windows

The Windows applications make heavy use of Microsoft's `windows-rs` crate for
all Windows methods and COM object implementation/handling.

- `windows/ime`: a Windows Text Services Framework (TSF) DLL application that
  provides the actual input method implementation.
- `windows/settings`: a basic Settings application (using Win32 PropSheets) that
  allows the user to configure the settings for the IME.
- `windows/installer`: TODO

### DLL Registration

Prior to deveopment, you must uninstall any previously installed version of the
IME from the system. After building the `windows/ime` crate, you can manually
register the DLL file in an elevated PowerShell as follows:

```
cd target\debug
regsvr32.exe /s khiin_windows.dll    # /s for silent install
```

To register the x86 (32-bit) DLL, build the crate using:

```
cargo.exe build --target i686-pc-windows-msvc
```

Then use an elevated 32-bit `cmd.exe` prompt
(`C:\Windows\SysWOW64\cmd.exe`, not PowerShell) and run:

```
cd target\i686-pc-windows-msvc
C:\Windows\SysWOW64\regsvr32.exe khiin_windows.dll
```

You should unregister these DLLs when you are not actively developing:

```
regsvr32.exe /u khiin_windows.dll     # /u to unregister

# In a 32-bit cmd.exe
C:\Windows\SysWOW64\regsvr32.exe /u khiin_windows.dll
```

The registration command will run the DLL using the entry point
`ime/src/dll.rs#DllRegisterServer`. This function basically just writes all of
the necessary registry entries for Windows to recognize the DLL as an input
method and provide it in the input method picker in the system tray.

### TIP (Text Input Processor)

The Windows TSF (Text Services Framework) is an expansive and highly
over-engineered tool, at least for our use case. However, we need good TSF
integration to ensure that the IME works with as many applications as possible.

This entire application has been written from scratch, and while Rust and the
`windows-rs` crate make Windows COM programming much easier and less
error-prone, writing a TSF library is still no easy task. We referred to at
least half a dozen different open source TSF IMEs throughout development, since
Microsoft documentation is imprecise or out of date in many areas.

- [mozc/tip](https://chromium.googlesource.com/external/mozc/+/master/src/win32/tip)
- [microsoft/Windows-classic-samples](https://github.com/microsoft/Windows-classic-samples/tree/main/Samples/Win7Samples/winui/input/tsf/textservice)
- [dinhngtu/VietType](https://github.com/dinhngtu/VietType)
- [chewing/windows-chewing-tsf](https://github.com/chewing/windows-chewing-tsf)
- [EasyIME/libIME2](https://github.com/EasyIME/libIME2)
- [rime/weasel](https://github.com/rime/weasel/)
- [keymanapp/keyman](https://github.com/keymanapp/keyman/tree/master/windows/src/engine/kmtip)

Hopefully this application will also serve as a good reference point for others
who wish to build Windows TSF IMEs.

All TSF related code is found in `windows/ime/src/tip/` (TextInputProcessor).
Essentially, we implement the required COM interfaces to be able to receive
keystrokes from the operating system and then put our resulting text into the
currently focused text input. Sounds simple, but there is a lot going on.

- `TextService`: implements the main TSF interface `ITfTextInputProcessorEx`,
  among others. Also the main interface used to pass messages between different
  parts of the program.
- `EngineCoordinator`: connects to the actual processing `Engine` from the
  `khiin` crate.
- `CompositionMgr`: manipulates the in-line "pre-edit" text shown at the caret
  position in an application, including decorations like underlines for
  different states of input
- `CandidateListUI`: prepares data for and controls display of the
  `CandidateWindow`
- `KeyEventSink`: collect key events from the system, including regular keys and
  "preserved keys" (a.k.a. keyboard shortcuts registered with TSF)
- `EditSession`: obtains the `TfEditCookie` (session token). A new session token
  is required for every interaction with the composition. (Namely: setting,
  clearing, or measuring text, etc.)
- `KhiinClassFactory`: creates a `TextService` when the DLL is initialized by
  TSF

There are many other classes, most of which provide some minor but required
function, some of which don't seem to be required but are found in most example
IMEs, including Microsoft's sample IME. In any event, those classes listed above
are where most of the actual work happens.

The Windows app is only intended to support Windows 10 and above, and has not
been tested on Windows 7 or 8. There are almost definitely some API calls or
libraries used which are not available prior to Windows 10. If you want to
develop Windows 7/8 support, feel free to work on it but it might be a big
project for a small (and shrinking) user base. The Windows app includes a 32-bit
DLL only to support 32-bit applications on 64-bit Windows 10.

### Settings App

The settings app is a basic [property
sheet](https://docs.microsoft.com/en-us/windows/win32/controls/property-sheets)
application, with a few dialog boxes and standard Win32 controls.

Most settings are saved in `khiin_config.ini`, which is then read by the DLL to
load the configuration options.

Some settings are saved in the registry, but we will migrate these so that
everything is saved in the `ini`.

### WiX Installer

TODO

---

## Android App

The Android IME is currently in progress / unstable. It is a modern Jetpack
Compose app written in Kotlin. The `android/rust` folder contains a small
wrapper around `khiin` that communicates with the Android app via JNI.

There is currently a
[bug](https://github.com/mozilla/rust-android-gradle/issues/105) that crashes
the Android app on an x86_64 emulator. It works on the x86 emulator, and on the
ARM devices themselves. If the linked issue is ever fixed we will update to make
that work. (Although currently we are using
[`android-rust`](https://github.com/MatrixDev/GradleAndroidRustPlugin) instead
of `rust-android-gradle`; the issue is related to the NDK and not the plugin.)

As of now the database must be built and placed in `android/app/src/main/assets`
manually, and it must be deleted from the device (or emulator) to be updated. We
will fix these to automate everything later.

---

## Development CLI App

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
