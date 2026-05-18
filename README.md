# WispLingua

A lightweight tray translator powered by LLMs. Select text in any app, press a hotkey, get a streaming translation in a popup near your cursor. Built with Tauri 2.

## Features

- Lives in the system tray. Around 30 to 80 MB of RAM at idle.
- Global hotkey with chord support: single, double or triple presses. Double Ctrl+C works the same way as in DeepL.
- Captures the current selection from any application via clipboard hijack, then restores your clipboard.
- Frameless popup that appears next to the cursor and is clamped to the working area on multi-monitor setups.
- Streams translations token by token instead of waiting for the whole response.
- OpenRouter adapter with a live model picker. Fetches the full catalog and lets you search through it.
- Bidirectional auto-detect. Type Russian, get English. Type English, get Russian. Configure the language pair once.
- Replace button. One click pastes the translation back over your original selection.
- API keys are stored in the OS keychain (Credential Manager on Windows, Keychain on macOS, Secret Service on Linux).
- Autostart on login.
- Works on Windows, macOS and Linux (X11).

## Install

Download the latest installer from the [Releases](../../releases) page.

| Platform | Asset |
| --- | --- |
| Windows | `WispLingua_x.y.z_x64-setup.exe` |
| macOS (Apple Silicon) | `WispLingua_x.y.z_aarch64.dmg` |
| macOS (Intel) | `WispLingua_x.y.z_x64.dmg` |
| Linux | `wisplingua_x.y.z_amd64.AppImage` or `.deb` |

## Build from source

Requirements:

- Node.js 20 or newer
- pnpm 10 or newer
- Rust 1.77 or newer
- Platform build dependencies as listed in the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

```sh
git clone https://github.com/your-username/wisplingua.git
cd wisplingua
pnpm install
pnpm tauri:build
```

The bundle ends up in `src-tauri/target/release/bundle/`. A portable executable also sits at `src-tauri/target/release/wisplingua.exe` (or the platform equivalent).

For development with hot reload:

```sh
pnpm tauri:dev
```

## Usage

On first launch the settings window opens. Paste your OpenRouter key under the Provider tab, hit Refresh model list to pull the catalog, pick a model. Set your primary and secondary language under the General tab. Close the window. The app stays in the tray.

Select text anywhere on screen, press your hotkey, and the popup appears next to the cursor with the streaming translation.

In the popup:

- Click the language code to swap the source and target.
- Hit the pin icon to keep the popup open after focus loss.
- Hit Replace at the bottom right to paste the translation over your original selection.
- Press Escape or click the X to close.

### Hotkey presets

| Preset | Description |
| --- | --- |
| Double Ctrl+C | The default. Reliable across almost every app. Same idea as DeepL. |
| Triple Ctrl | Three quick taps of Ctrl alone. No other key. |
| Ctrl+Alt+T | A classic single combo. |
| Ctrl+Shift+Space | Short and rarely conflicts. |

You can record any custom shortcut from the Hotkey tab and choose single, double or triple presses. The chord window (max delay between presses) is adjustable.

### Adapters

Shipped:

- OpenRouter (streaming, full model catalog)

Adding a new provider takes one trait implementation in `src-tauri/src/provider/`. PRs welcome.

## Platform notes

### Windows

Works out of the box. Code signing is not yet set up, so SmartScreen may warn on the very first launch of the installer. Click "More info" then "Run anyway".

### macOS

WispLingua needs Input Monitoring and Accessibility permissions to register the global hotkey and to simulate Ctrl+C. macOS prompts for these on first run. If you skip the prompts, open System Settings, Privacy and Security, and grant them manually.

The unsigned `.dmg` from CI will trigger Gatekeeper. Right click the app and choose Open the first time.

### Linux

Tested on X11. Under Wayland the global keyboard hook does not deliver events from other apps yet. That needs the global-shortcut portal, which is rolling out across compositors.

The Debian package depends on `libwebkit2gtk-4.1-0`, `libgtk-3-0`, `libayatana-appindicator3-1` and `libxdo3`. The AppImage bundles its own runtime.

## Configuration

Plain config lives in:

- Windows: `%APPDATA%\com.wisplingua.app\config.json`
- macOS: `~/Library/Application Support/com.wisplingua.app/config.json`
- Linux: `~/.config/com.wisplingua.app/config.json`

API keys live in the OS keychain under service name `wisplingua`. Nothing sensitive ends up on disk in plain text.

## Project layout

```
.
├── src/                React popup and settings UI
│   ├── popup/          The floating translation window
│   ├── settings/       The settings window (general, hotkey, provider, appearance)
│   └── shared/         Components, hooks, store, types
├── src-tauri/          Rust backend
│   └── src/
│       ├── hotkey/     Chord engine + rdev global listener
│       ├── capture/    Clipboard hijack and paste
│       ├── provider/   Trait and adapters
│       ├── translator/ Pipeline that ties capture, provider and popup together
│       ├── desktop.rs  Platform helpers (Win32 focus tracking)
│       └── ...
└── .github/workflows/  CI for Windows, macOS, Linux
```

## Roadmap

- More provider adapters: OpenAI, Anthropic, DeepL, Ollama
- OCR fallback for non selectable text (PDFs, images, protected windows)
- Translation history with search
- Native macOS and Linux notarization
- Wayland support via xdg-desktop-portal

## Contributing

PRs and issues are welcome. Before opening a large PR, please file an issue so we can discuss the approach.

## License

MIT. See [LICENSE](LICENSE).
