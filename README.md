# rich

A CLI tool that converts Markdown text in your clipboard to rich text (HTML).

## Usage

1. Copy Markdown text to your clipboard
2. Run `rich`
3. Paste anywhere that supports rich text (Word, Outlook, Slack, etc.)

```bash
rich
```

## Installation

```bash
cargo install --path .
```

## Build from source

```bash
cargo build --release
```

Binary will be at `target/release/rich.exe` (Windows) or `target/release/rich` (Linux/macOS).
