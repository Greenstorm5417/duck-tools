# DuckyScript Language Support

DuckyScript language support for VS Code.

This extension uses the `ducky-lsp` language server for diagnostics, completions, hover help, formatting, and more.

> Disclaimer: This project is an independent, unofficial work and is not affiliated with, endorsed by, sponsored by, or associated with Hak5, USB Rubber Ducky, Payload Studio, or any related products or trademarks.

## Features

- Syntax highlighting
- LSP-based diagnostics
- Completions
- Hover documentation
- Go to definition / find references
- Formatting

## Requirements

- Install `ducky-lsp` (required)
- Install `duck` CLI (optional, required for the `DuckyScript: Compile Current File` command)

## Install ducky-lsp

Windows (PowerShell):

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/Greenstorm5417/duck-tools/releases/latest/download/ducky-lsp-installer.ps1 | iex"
```

macOS/Linux:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/Greenstorm5417/duck-tools/releases/latest/download/ducky-lsp-installer.sh | sh
```

## Install duck CLI

Windows (PowerShell):

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/Greenstorm5417/duck-tools/releases/latest/download/ducky-cli-installer.ps1 | iex"
```

macOS/Linux:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/Greenstorm5417/duck-tools/releases/latest/download/ducky-cli-installer.sh | sh
```

## Configuration

### LSP path

The extension tries to find `ducky-lsp` automatically (PATH + standard install locations).

If needed, set a custom path:

```json
{
  "duckyscript.lsp.path": "C:\\Program Files\\ducky-lsp\\bin\\ducky-lsp.exe"
}
```

### CLI path

The compile command uses `duck build -i <file>`.

If `duck` is not in PATH, set a custom path:

```json
{
  "duckyscript.cli.path": "C:\\Program Files\\ducky-cli\\bin\\duck.exe"
}
```

## Commands

- `DuckyScript: Compile Current File` (`Ctrl+Shift+B`)

## Links

- Repository: https://github.com/Greenstorm5417/duck-tools
- Issues: https://github.com/Greenstorm5417/duck-tools/issues
