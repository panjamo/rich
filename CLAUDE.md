# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust CLI tool that reads Markdown text from the system clipboard, converts it to HTML, and writes it back as rich text (HTML format with plain text fallback).

## Build Commands

```bash
cargo build              # Debug build
cargo build --release    # Release build
cargo run                # Build and run
cargo clippy --fix       # Fix lint warnings
cargo fmt                # Format code
cargo test               # Run tests
```

## Architecture

Single-file application (`src/main.rs`) with straightforward flow:
1. Read text from clipboard via `arboard`
2. Convert Markdown to HTML via `markdown` crate
3. Set clipboard with HTML content (rich text) and original Markdown as fallback
