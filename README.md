# Duck Tools

<p align="center">
  <img src="duck-icon.svg" alt="Duck Tools Logo" width="200" height="200">
</p>

Duck Tools is a lightweight, unofficial toolchain for working with **DuckyScript**, including a fast CLI compiler and editor tooling.

It aims to be simple, predictable, and easy to integrate into existing workflows.

---

## Quick Start

### Install CLI

**macOS / Linux**
```bash
curl -LsSf https://github.com/Greenstorm5417/duck-tools/releases/latest/download/ducky-cli-installer.sh | sh
````

**Windows**

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/Greenstorm5417/duck-tools/releases/latest/download/ducky-cli-installer.ps1 | iex"
```


### (Optional) Editor Support

For diagnostics, formatting, and autocomplete, install the **DuckyScript VS Code extension**.
It requires `ducky-lsp` to be installed from the same releases page.

---

## Usage

### Initialize a Project

Create a new DuckyScript project with configuration:

```bash
duck init
```

This creates:
- `duck.toml` - Configuration file for your project
- `helloworld.txt` - Example DuckyScript file

The `duck.toml` file contains:

```toml
[workspace]
main_file = "helloworld.txt"

[formatter]
enabled = true
indent_size = 4
use_tabs = true
max_line_length = 120

[linter]
enabled = true
check_line_length = true
check_trailing_whitespace = true
```

### Compile DuckyScript

Compile your script to `inject.bin`:

```bash
# Compile the main_file from duck.toml
duck build

# Or specify a file explicitly
duck build -i payload.txt -o inject.bin

# Show compilation statistics
duck build --stats

# Output as hex instead of binary
duck build --hex
```

Output example:
```
Successfully compiled payload.txt into inject.bin
  Version: DuckyScript 3.0
  Size: 245 bytes (1.5% of 16KB)
  Compile time: 12.34ms
  SHA256: a1b2c3d4...
```

### Format Code

Auto-format your DuckyScript files:

```bash
# Format the main_file from duck.toml
duck fmt

# Format specific files
duck fmt script1.txt script2.txt

# Check formatting without writing changes
duck fmt --dry-run

# Verbose output showing all changes
duck fmt --verbose
```

The formatter will:
- Normalize indentation (tabs or spaces)
- Trim trailing whitespace
- Ensure consistent spacing
- Add final newline

### Lint Code

Check your code for issues:

```bash
# Lint the main_file from duck.toml
duck lint

# Lint specific files
duck lint payload.txt

# Show detailed output
duck lint --verbose
```

The linter checks for:
- Line length violations
- Trailing whitespace
- Mixed indentation (tabs and spaces)
- Suspicious delay values
- Missing final newline

Example output:
```
Linting issues in payload.txt:
  15:80 [WARN] Line exceeds maximum length of 120 characters (max-line-length)
  23:45 [INFO] Trailing whitespace detected (no-trailing-whitespace)

Linted 1 file(s)
Found 2 issue(s) in 1 file(s):
  0 error(s)
  1 warning(s)
  1 info(s)
```

### Configuration

Edit `duck.toml` to customize behavior:

```toml
[workspace]
main_file = "payload.txt"  # Default file for build/fmt/lint

[formatter]
enabled = true
indent_size = 4           # Spaces per indent level
use_tabs = true           # Use tabs instead of spaces
max_line_length = 120     # Maximum line length
trim_trailing_whitespace = true
insert_final_newline = true

[linter]
enabled = true
check_line_length = true
max_line_length = 120
check_trailing_whitespace = true
check_mixed_indentation = true
check_suspicious_delays = true
suspicious_delay_threshold = 10000  # Warn on delays > 10s
require_final_newline = true
```

### Version Information

Check installed version:

```bash
duck version
```

Output:
```
duck 0.1.0
DuckyScript toolchain

Components:
  compiler:  0.1.0
  formatter: 0.1.0
  linter:    0.1.0
  lsp:       0.1.0
```

---

## Disclaimer

This project is **independent and unofficial**.

It is **not affiliated with, endorsed by, sponsored by, or associated with** Hak5, USB Rubber Ducky, Payload Studio, or any related products or trademarks.
All names are used for compatibility and descriptive purposes only.

