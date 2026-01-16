# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Alfred Zed Explorer is a high-performance Alfred workflow written in Rust that enables searching and opening Zed projects. It provides three main commands: `zopen` (search project directories), `zrecent` (search recent projects from Zed's database), and `zed` (unified search combining both).

## Build and Development Commands

### Essential Commands
- `make install-deps` - Install Rust dependencies (first-time setup)
- `make test` - Run all tests
- `make lint` - Run clippy and format checks
- `make fmt` - Format code
- `make release-lint` - Build optimized release with linting
- `make package` - Build and package workflow for distribution
- `make install-local` - Install to Alfred for local testing
- `make debug-zed-db` - Debug tool to list projects from Zed database

### Testing Individual Components
- `cargo test` - Run all Rust tests
- `cargo test -p <test_name>` - Run specific test
- `./target/release/debug-zed-db` - Test Zed database reading

## Architecture Overview

### Core Components

1. **Binary Targets** (src/bin/):
   - `search.rs` - `zopen` command: searches project directories
   - `recent.rs` - `zrecent` command: searches Zed's recent projects database
   - `zed.rs` - `zed` command: unified search (combines both sources)
   - `debug.rs` - Debug tool for Zed database inspection

2. **Library Modules** (src/):
   - `lib.rs` - Alfred JSON structures and output formatting
   - `project.rs` - Project detection logic (identifies project types by files)
   - `zed_db.rs` - SQLite database reading for recent projects

### Key Design Patterns

1. **Alfred Integration**: All commands output JSON in Alfred's format via `AlfredOutput` struct
2. **Project Detection**: Uses file presence (`.git`, `package.json`, `Cargo.toml`, etc.) to identify project types
3. **Fuzzy Search**: Uses `fuzzy-matcher` crate for performant fuzzy searching
4. **Database Access**: Reads Zed's SQLite database from multiple possible paths

### Environment Variables

- `PROJECT_DIRS` - Colon-separated list of custom project directories to search

## Project Detection Types

The workflow detects these project types by checking for specific files:
- Rust: `Cargo.toml`
- Node.js: `package.json`
- Python: `requirements.txt`, `setup.py`, `pyproject.toml`
- Go: `go.mod`
- Git: `.git` directory
- And several others (see `src/project.rs` for full list)

## Release Process

GitHub Actions automatically builds for both ARM64 and x86_64 macOS on push to master. The workflow creates versioned `.alfredworkflow` files as release artifacts.