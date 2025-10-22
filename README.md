<img align="left" width="80" height="80" src="assets/icon.png">

# ZokuZoku-Edge
ZokuZoku-Edge is a Visual Studio Code extension which acts as the main translation tool for Hachimi. It provides a number of custom editors that are specialized for editing the various JSON dict formats that Hachimi uses, allowing translators to work on translations without having to edit the JSON file directly.

[![Extension Builder](https://github.com/thshafi170/ZokuZoku-Edge/actions/workflows/build-and-release.yml/badge.svg?branch=main)](https://github.com/thshafi170/ZokuZoku-Edge/actions/workflows/build-and-release.yml)

# Features
- **No fuss installation:** Just install the extension, and it will guide you through the setup process. No command prompt needed.
- **No preprocessing or postprocessing:** All data used by ZokuZoku is dynamically generated the moment you access them, no separate data generation process or full data download required, and it will keep itself up-to-date with the game's assets without any manual actions. Translated data you made with ZokuZoku is in the exact same format as Hachimi uses, so they can be loaded in-game directly without any postprocessing or patching step.
- **User friendly interface:** ZokuZoku provides tree views that lists translatable assets in a logical manner, which helps you navigate and find what you want to translate easily. Its custom editors also resembles the main VSCode interface, so long time users will feel right at home, while also being easy to pick up by new users.
- **Streamlined editing:** Story translations are now easier than ever with Hachimi's auto wrapping system and ZokuZoku's accurate story previews. Proper text wrapping is enforced with this system by default, so you could focus more on translating the content and less on making that content fit in-game.
- **Powered by Visual Studio Code:** VSCode is a mature text editor that just happens to be specifically tailored towards coding; so it makes perfect sense for it to be editing JSON files. ZokuZoku is built on top of this tried and tested platform with a lot of features that are perfect for translation work and collaboration, such as Git integration, so you could do everything without ever leaving the editor.

# Installation
Download the .vsix file for the latest version on the [Releases](https://github.com/thshafi170/ZokuZoku-Edge/releases) page. To install it, open the Extensions panel in VSCode, click on the 3 dots button on the top right, choose "Install from VSIX..." and select the file you just downloaded.

### Prerequisites
- OS: Windows 10+ or Linux x64. macOS is not officially supported but might work with some special setup.
- Visual Studio Code v1.90 or later.
- UM:PD installed from DMM or files from the Windows/Android version.

# Getting started
TODO

# Development
*Please use the pnpm package manager while working on this project.*

To get started, install the dependencies in the root directory and also the `webviews` directory, as there are two separate project trees: one for the extension part, and the other for the editors.

ZokuZoku uses a special Python installation for Node.js called [`pymport`](https://github.com/mmomtchev/pymport), and in development mode, it looks for Python modules in its own directory. To install the required dependencies, instead of following the usual setup procedure, run `npx pympip3 install UnityPy==1.10.18`.

After that, you can work on the project just like you would with any other VSCode extension.

# Automated Builds and Releases

This project uses GitHub Actions to automatically build and release the VSCode extension whenever changes are pushed to the main branch. The workflow:

1. **Builds the extension** - Compiles TypeScript, builds webviews, and packages the extension
2. **Auto-increments version** - Automatically increments the edge version number (e.g., `0.3.2-edge` → `0.3.2-edge1` → `0.3.2-edge2`)
3. **Creates GitHub releases** - Automatically creates releases with the packaged `.vsix` file
4. **Commits version changes** - Updates the version in `package.json` and commits the change back to the repository

The workflow is triggered on:
- Push to `main` branch (creates/updates release with incremented version)
- Pull requests to `main` branch (builds and tests only)

You can find the workflow configuration in `.github/workflows/build-and-release.yml`.

# Acknowledgement
- [Main ZokuZoku Repository](https://github.com/Hachimi-Hachimi/ZokuZoku) maintained by @Hachimi-Hachimi Team
- [Working ZokuZoku Repository](https://github.com/Mario0051/ZokuZoku) maintained by @Mario0051
- [Hachimi Project Discord](https://discord.gg/BVEt5FcxEn)

# License
[GNU GPLv3](LICENSE)
