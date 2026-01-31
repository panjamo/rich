# rich

A CLI tool that converts Markdown text to rich text (HTML) and places it on your clipboard.

## Usage

### From Clipboard

1. Copy Markdown text to your clipboard
2. Run `rich`
3. Paste anywhere that supports rich text (Word, Outlook, Slack, etc.)

```bash
rich
```

### From STDIN

Pipe Markdown content directly into the tool:

```bash
echo "# Hello World" | rich

cat README.md | rich

# Convert and paste a file's content as rich text
cat notes.md | rich
```

The converted rich text is placed on your clipboard, ready to paste.

## Installation

```bash
cargo install --path .
```

## Build from source

```bash
cargo build --release
```

Binary will be at `target/release/rich.exe` (Windows) or `target/release/rich` (Linux/macOS).
