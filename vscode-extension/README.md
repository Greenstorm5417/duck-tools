# DuckyScript Language Support

Full language support for DuckyScript 3.0 with syntax highlighting, LSP features, and more.

## Features

- 🎨 **Syntax Highlighting** - Full DuckyScript 3.0 syntax support
- 🔍 **Diagnostics** - Real-time error and warning detection
- 💡 **Auto-completion** - Smart completions for commands, variables, and keywords
- 📖 **Hover Information** - Documentation on hover
- 🔗 **Go to Definition** - Jump to variable/label definitions
- 🔎 **Find References** - Find all usages of variables/labels
- ✏️ **Rename Symbol** - Rename variables/labels across file
- 📋 **Document Outline** - Navigate code structure
- 🔧 **Code Actions** - Quick fixes for common issues
- 🎯 **Code Lenses** - Inline reference counts
- 📝 **Formatting** - Auto-format your code
- ⚡ **And much more!**

## Installation

### 1. Install the Duck Toolchain

The extension requires the `ducky-lsp` language server, which is installed with the duck toolchain.

**Windows (PowerShell):**
```powershell
irm https://github.com/Greenstorm5417/duck-tools/releases/latest/download/install.ps1 | iex
```

**macOS/Linux:**
```bash
curl -L https://github.com/Greenstorm5417/duck-tools/releases/latest/download/install.sh | sh
```

### 2. Install the VS Code Extension

Search for "DuckyScript" in the VS Code Extensions marketplace and install.

## Configuration

The extension will automatically find the `ducky-lsp` binary if it's in your PATH.

### Custom LSP Path

If the LSP is not found automatically, configure it in your `settings.json`:

```json
{
  "duckyscript.lsp.path": "/path/to/ducky-lsp"
}
```

**Default locations:**
- Windows: `%USERPROFILE%\.duck\bin\ducky-lsp.exe`
- macOS/Linux: `~/.duck/bin/ducky-lsp`

### LSP Features

Enable/disable specific LSP features:

```json
{
  "duckyscript.lsp.enable": true,
  "duckyscript.lsp.hover.enable": true,
  "duckyscript.lsp.completion.enable": true,
  "duckyscript.lsp.diagnostics.enable": true,
  "duckyscript.lsp.formatting.enable": true
}
```

## Usage

### Compile Current File

- **Keyboard**: `Ctrl+Shift+B` (Windows/Linux) or `Cmd+Shift+B` (macOS)
- **Command Palette**: `DuckyScript: Compile Current File`
- **Editor Title Bar**: Click the ▶️ play button

### Format Document

- **Keyboard**: `Shift+Alt+F` (Windows/Linux) or `Shift+Option+F` (macOS)
- **Command Palette**: `Format Document`

## Commands

- `duck build` - Compile DuckyScript to inject.bin
- `duck fmt` - Format DuckyScript files
- `duck lint` - Lint DuckyScript files
- `duck init` - Initialize a new project with duck.toml
- `duck version` - Show version information
- `duck update` - Update to latest version

## File Association

The extension activates for `.txt` files containing DuckyScript code.

## Troubleshooting

### LSP Not Starting

1. Verify `ducky-lsp` is installed:
   ```bash
   ducky-lsp --version
   ```

2. Check the Output panel: `View → Output → DuckyScript Language Server`

3. Manually set the LSP path in settings

### Compilation Issues

Make sure `duck` is in your PATH:
```bash
duck --version
```

## Links

- [GitHub Repository](https://github.com/Greenstorm5417/duck-tools)
- [Report Issues](https://github.com/Greenstorm5417/duck-tools/issues)
- [DuckyScript Documentation](https://docs.hak5.org/hak5-usb-rubber-ducky)

## License

MIT License - See LICENSE file for details
