# Khíín Desktop App

This is the companion app for the IME on Windows and macOS. It allows users to
configure the IME and set up their custom dictionary.

## Development

This app is built with [Tauri](https://tauri.app/) and
[Svelte](https://svelte.dev/).

Prerequisites: `node` and `npm`

Quick start:

```bash
cargo make tauri-dev
```

Note: the first build can take quite a while if you don't have `tauri-cli`
already and need to build it.

Or if you want to do it step by step:

```bash
cargo install --force tauri-cli
cargo tauri icon app/frontend/static/app-icon.png
(cd app/frontend && npm i)
cargo tauri dev
```

For debugging, run `npm run dev` in a separate terminal, and then launch the
`Tauri Dev` launch configuration from VSCode.

We recommend using `nvm` to install `node` and `npm`. On Unix:

```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash
nvm install --lts
nvm use --lts
```

On Windows: [`nvm-windows`](https://github.com/coreybutler/nvm-windows)

### Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) +
[Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
+
[rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
