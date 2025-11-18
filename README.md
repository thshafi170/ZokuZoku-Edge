<img align="left" width="80" height="80" src="assets/icon.png">

# ZokuZoku-Edge

A VS Code extension for translating UM:PD using Hachimi's translation framework.

![Extension Builder](https://github.com/thshafi170/ZokuZoku-Edge/actions/workflows/builder.yml/badge.svg)

> [!CAUTION]
> This repository is for extension building only and **does not accept pull requests**. For feature improvements, bug fixes, or any contributions, please submit your PRs to the [active upstream repository](https://github.com/Mario0051/ZokuZoku) maintained by @Mario0051.

## Features

- Custom editors for story timelines, lyrics, localize dict, and MDB text
- Tree views for navigating stories and translation files
- Hachimi IPC integration for live reloading

## Installation

### Download

Get the latest `.vsix` file from [Releases](https://github.com/thshafi170/ZokuZoku-Edge/releases).

### Install in VS Code

**Method 1: Via Extensions Panel**
1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X / Cmd+Shift+X)
3. Click the ⋯ (three dots) menu at the top-right
4. Select "Install from VSIX..."
5. Browse and select the downloaded `.vsix` file
6. Restart VS Code when prompted

**Method 2: Via Command Line**
```bash
code --install-extension zokuzoku-edge-<version>.vsix
```

**Method 3: Drag and Drop**
1. Open VS Code
2. Drag the `.vsix` file into the VS Code window
3. Confirm the installation
4. Restart VS Code when prompted

## Prerequisites

### System Requirements

- **Windows:** Windows 10/11 with Visual C++ Redistributables (2015-2022)
- **Linux:** x64 with SQLite3 installed
- **macOS:** SQLite3 (usually pre-installed at `/usr/bin/sqlite3`)
- **VS Code:** v1.90 or later
- **Python:** 3.12+

### UM:PD Game Files

- DMM or Steam installation (JP version)
- Steam installation (Global version)
- Android client files (may require manual meta decryption)

### SQLite3 Installation (Linux)

```bash
# Debian/Ubuntu
sudo apt install sqlite3

# Fedora/RHEL
sudo dnf install sqlite

# Arch Linux
sudo pacman -S sqlite

# openSUSE
sudo zypper install sqlite3
```

**NixOS:** Add to `/etc/nixos/configuration.nix`:
```nix
environment.systemPackages = with pkgs; [
  sqlite
];
```
Then run: `sudo nixos-rebuild switch`

### SQLite3 on macOS

macOS usually ships with SQLite3. If needed, install via Homebrew:
- Apple Silicon: `/opt/homebrew/bin/sqlite3`
- Intel: `/usr/local/bin/sqlite3`

Configure via Settings → `zokuzoku.sqlite3` if not on PATH.

## Usage

### First-Time Setup

1. **Open your translation project** containing `localized_data/config.json`
   - The extension activates automatically when this file is detected

2. **Install dependencies** (first launch only)
   - A notification will appear: "ZokuZoku needs to install some dependencies before it can be used"
   - Click **OK** to install required Python packages
   - Wait for installation to complete

3. **Open ZokuZoku panel**
   - Click the **ZokuZoku icon** in the Activity Bar (left sidebar)
   - You'll see "ZOKUZOKU: INACTIVE" status

4. **Enable ZokuZoku**
   - Click the **Enable** button, or
   - Open Command Palette (Ctrl+Shift+P / Cmd+Shift+P) → "ZokuZoku: Enable"

5. **Configure settings** (File → Preferences → Settings)
   - `zokuzoku.gameDataDir` - Path to game's data directory
   - `zokuzoku.localizeDictDump` - Path to `localize_dump.json`

### Working with Translations

Once enabled, the ZokuZoku sidebar shows:
- **All stories** - Browse all story timelines
- **Home dialogues** - Character home screen dialogues  
- **Main stories** - Main story arcs
- **Localize dict** - Translation dictionary editor
- **MDB** - Master database text entries
- **Lyrics** - Song lyrics editor
- **Hachimi Controls** - Live reload and folder management

## Development

```bash
pnpm install
cd webviews && pnpm install
pnpm run watch
```

Press F5 to debug.

## Troubleshooting

**SQLite3 not found (Linux):**
```bash
# Debian/Ubuntu
sudo apt install sqlite3

# Arch
sudo pacman -S sqlite
```

Or set `zokuzoku.sqlite3` in settings to the full path.

**Performance issues:** Use "ZokuZoku: Clear cache" command.

# Acknowledgement
- [Main ZokuZoku Repository](https://github.com/Hachimi-Hachimi/ZokuZoku) maintained by @Hachimi-Hachimi Team
- [Active ZokuZoku Repository](https://github.com/Mario0051/ZokuZoku) maintained by @Mario0051
- [Hachimi Project Discord](https://discord.gg/BVEt5FcxEn)

# License
[GNU GPLv3](LICENSE)
