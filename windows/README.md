# Khíín for Windows

The Windows applications make heavy use of Microsoft's `windows-rs` crate for
all Windows methods and COM object implementation/handling.

- `windows/ime`: a Windows Text Services Framework (TSF) DLL application that
  provides the actual input method implementation.
- `windows/settings`: a basic Settings application (using Win32 PropSheets) that
  allows the user to configure the settings for the IME.
- `windows/installer`: TODO

## Development

At present we are mainly using Visual Studio Code to work on the Windows apps.
There are appropriate task and launch configurations for Windows in the
`.vscode` folder checked in to this repository.

The best way to debug the Windows app is to launch Notepad and attach a debugger
to that. Note that during development, and especially for debugging, it is
highly recommended to set the variable:

```rust
// ime/src/dll.rs
static ENABLE_DEBUG_LOCK: bool = true;
```

You will need to open the registry editor and manually clear the key in order to
turn the IME on again after the first use. The reason for this is simple: once
the IME is registered with the system (see the next section), Windows will
automatically load the IME DLL into every foreground application if the IME is
active. This means that if you open the IME in an app (like Notepad), and hit a
debug breakpoint in VS Code, the IME will attach to VS Code as soon as the
breakpoint is hit and the app moves into the foreground. Rebuilding the DLL then
requires closing VS Code and reopening it, in order to remove the lock on the
DLL file.

Setting the `ENABLE_DEBUG_LOCK` flag prevents the DLL from attaching to more
than one process at a time.

### DLL Registration

Prior to deveopment, you must uninstall any previously installed version of the
IME from the system. After building the `ime` crate, you can manually
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
`src/dll.rs::DllRegisterServer()`. This function basically just writes all of
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

All TSF related code is found in `ime/src/tip/` (TextInputProcessor).
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