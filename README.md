# Alfred Zed Explorer

A powerful Alfred workflow to quickly search and open Zed projects, written in Rust for maximum performance. Inspired by the popular VSCode Workspace Explorer workflow.

## ✨ Features

- 🔍 **Project Search**: Search and open any project in your workspace (`zed` keyword)
- ⏰ **Recent Projects**: Quickly access recently opened projects (`zedr` keyword)
- 🎯 **Smart Detection**: Automatically detects project types (Node.js, Rust, Python, Go, etc.)
- 🔧 **Dynamic Zed DB Detection**: Automatically finds Zed databases in all valid paths (e.g., `0-preview`, `123-global`, `456-stable`)
- 📁 **Custom Directories**: Configure custom project root directories via `PROJECT_DIRS` environment variable
- ⚡ **Blazing Fast**: Written in Rust for optimal performance with fuzzy search
- 🎨 **Visual Icons**: Shows project type icons for easy identification
- 🦾 **Robust**: Cross-platform support (macOS, Linux) with multiple Zed installation methods

## 📋 Requirements

- **Alfred** with Powerpack
- **Zed** editor (CLI must be installed)
- **Rust** (for building from source)

## 🔧 Installation

### Method 1: Download Pre-built Release (Recommended)

1. Download the latest release from the [releases page](https://github.com/fanwenlin/alfred-zed-explorer/releases)
2. Double-click `OpenInZed.alfredworkflow` to install
3. Alfred will automatically import the workflow

**Workflow Icon**: Features the official Zed logo (courtesy of Zed.dev)

### Method 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/fanwenlin/alfred-zed-explorer.git
cd alfred-zed-explorer

# Install dependencies and build
make install-deps
make release-lint

# Package the workflow for installation
make package

# The workflow will be on your Desktop - double-click to install
```

### Method 3: Local Development Install

For development and testing:

```bash
# After building with 'make release-lint'
make install-local

# This installs directly to Alfred's workflow directory
# No need to double-click or drag - immediately available!
```

## 🚀 Usage

### Search All Projects (`zopen`)

Type `zopen` followed by your search query to search for projects across all configured directories:

```
zopen myproject
```

### Search Recent Projects (`zrecent`)

Type `zrecent` to see and search through recently opened projects from Zed's database and custom directories:

```
zrecent
```

### Opening Projects

Simply press `Enter` on any project to open it in Zed.

## ⚙️ Configuration

### Custom Project Directories

You can customize the directories that are searched for projects by setting the `PROJECT_DIRS` environment variable in the workflow configuration.

1. Open Alfred Preferences
2. Go to Workflows → "Zed Workspace Explorer"
3. Click the `[x]` button in the top right
4. Set the `PROJECT_DIRS` variable

**Example:**
```
/Users/yourname/Projects,/Users/yourname/Work,/Users/yourname/SideProjects
```

### Default Directories

If no custom directories are configured, the workflow will automatically search these default locations:

- `~/Projects`
- `~/Code`
- `~/Developer`
- `~/GitHub`
- `~/Development`
- `~/Sites`
- `~/workspace`

### Project Detection

A directory is considered a project if it contains any of the following:

- `.git/` directory (Git repository)
- `package.json` (Node.js)
- `Cargo.toml` (Rust)
- `pyproject.toml` or `requirements.txt` (Python)
- `go.mod` (Go)
- `composer.json` (PHP)
- `Gemfile` (Ruby)
- `pom.xml` or `build.gradle` (Java)
- `Makefile`, `CMakeLists.txt`
- `*.xcodeproj`, `*.xcworkspace` (Xcode)
- `*.sln` (Visual Studio)

### Project Icons

The workflow shows different icons based on project type:

- 🟢 **Node.js projects** (with `package.json`)
- 🟤 **Rust projects** (with `Cargo.toml`)
- 🔵 **Python projects** (with `pyproject.toml` or `requirements.txt`)
- 🟢 **Go projects** (with `go.mod`)
- 🟣 **PHP projects** (with `composer.json`)
- 🔴 **Ruby projects** (with `Gemfile`)
- 🟠 **Git repositories** (other Git projects)
- 📁 **Generic folders**

### Zed Database Detection

The workflow automatically detects and reads all valid Zed database paths:

- `~/Library/Application Support/Zed/db/0-preview/db` (macOS Preview)
- `~/Library/Application Support/Zed/db/123-global/db` (macOS Global)
- `~/Library/Application Support/Zed/db/456-stable/db` (macOS Stable)
- `~/.local/share/zed/db/0-preview/db` (Linux Preview)
- And any other `\d+-(preview|global|stable)` pattern

## 🛠️ Development

### Project Structure

```
alfred-zed-explorer/
├── Cargo.toml              # Rust project configuration
├── Makefile               # Build automation
├── info.plist             # Alfred workflow configuration
├── zed-logo.png           # Zed official logo
├── src/
│   ├── lib.rs            # Library code
│   ├── project.rs        # Project detection logic
│   ├── zed_db.rs         # Zed database reading
│   └── bin/
│       ├── search.rs     # `zopen` command implementation
│       ├── recent.rs     # `zrecent` command implementation
│       └── debug.rs      # Debug tool for Zed DB
└── README.md
```

### Available Make Commands

```bash
make install-deps    # Install Rust dependencies
make build           # Build debug version
make release         # Build optimized release version
make release-lint    # Build with linting (recommended)
make test            # Run tests
make lint            # Lint code with clippy
make fmt             # Format code
make package         # Build release and package workflow
make install-local   # Install to Alfred for testing
make clean           # Clean build artifacts
make clean-all       # Clean everything including Desktop workflow
make pre-commit      # Run all pre-commit checks
make help            # Show help
```

### Building for Release

```bash
# Full release build with all checks
make release-lint

# Package for distribution
make package

# The workflow will be on your Desktop
# Ready to distribute or install
```

### Testing Local Changes

For rapid development:

```bash
# Make your changes
# Then run:
make release && make install-local

# The workflow is immediately available in Alfred
# No need to re-import or restart Alfred
```

### Cross-Platform Notes

The workflow automatically detects:

- **macOS**: `~/Library/Application Support/Zed/`
- **Linux**: `~/.local/share/zed/` or Flatpak installations
- **Zed installations**: Official, Preview, or custom builds

## 🐛 Troubleshooting

### Zed CLI Not Found

If you get an error that the `zed` command is not found:

1. Install Zed from https://zed.dev/
2. Open Zed
3. Open the command palette (`Cmd+Shift+P` or `Ctrl+Shift+P`)
4. Run "Install CLI"
5. Restart your terminal
6. Test: `which zed` should show the path

### Projects Not Showing Up

1. Verify your project directories exist
2. Check the `PROJECT_DIRS` environment variable is set correctly
3. Make sure your projects have project indicators (`.git`, `package.json`, etc.)
4. Try running the binary directly for debugging:
   ```bash
   ./target/release/zed-search
   ./target/release/zed-recent
   ```

### Slow Performance

If searching feels slow:

1. Reduce the number of directories in `PROJECT_DIRS`
2. The workflow already limits search depth (3 levels for `zed`, 2 for `zedr`)
3. Build an optimized release: `make release-lint`
4. Consider excluding very large directories via `.gitignore` or project structure

### Recent Projects Not Showing

1. Ensure Zed is actually opening projects (not just files)
2. Check Zed's database exists: `ls -la ~/Library/Application\ Support/Zed/db/`
3. Try the `zedr` command without query to see all projects
4. Note: `zedr` shows both Zed database projects AND custom directory projects

### Build Errors

Ensure you have Rust installed:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Or update:
rustup update

# Then run:
make install-deps
```

## 🔍 How It Works

### Architecture

1. **Rust Binaries**: Two compiled binaries provide the core functionality
   - `zed-search`: Searches all projects in configured directories
   - `zed-recent`: Gets recent projects from Zed DB + custom directories

2. **Alfred Integration**: Alfred workflow configuration calls the binaries and opens projects
   - Script filters parse JSON output from Rust binaries
   - Alfred's native "Open File" action opens folders with Zed

3. **Smart Discovery**:
   - Custom directory scanning with project type detection
   - Dynamic Zed database discovery (all `\d+-(preview|global|stable)` paths)
   - Fuzzy matching for responsive search

### Performance Optimizations

- **Compiled Rust**: Native performance vs interpreted scripts
- **Fewer Binaries**: Removed custom open script, using Alfred's native action
- **Efficient Scanning**: `walkdir` with depth limits and early filtering
- **Fuzzy Matching**: Skim matcher for responsive search
- **Deduplication**: Hash sets prevent duplicate entries
- **Native Opening**: Alfred's built-in "Open File" action is faster and more reliable

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`make test`)
5. Run lints (`make lint`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## 📄 License

MIT License - see LICENSE file for details

## 🙏 Credits

Inspired by [VSCode Workspace Explorer](https://github.com/Acidham/alfred-vscode-workspace-explorer) for Alfred.

Built with:
- [Rust](https://www.rust-lang.org/)
- [serde](https://serde.rs/) for JSON serialization
- [rusqlite](https://github.com/rusqlite/rusqlite) for SQLite database reading
- [fuzzy-matcher](https://github.com/lotabout/fuzzy-matcher) for fuzzy search
- [walkdir](https://github.com/BurntSushi/walkdir) for directory traversal

## 📊 Changelog

### v2.1.0 (Current)

- **Native Alfred Opening**: Replaced custom bash script with Alfred's built-in "Open File" action
  - More reliable and faster project opening
  - Reduced binary size (removed `zed-open` binary)
  - Better error handling through Alfred's native action
  - Follows Alfred best practices

### v2.0.0

- **Major Rewrite**: Completely rewritten in Rust for maximum performance
- **Dynamic Zed DB Detection**: Automatically discovers all Zed database paths
- **Enhanced Recent Projects**: Combines Zed DB + custom directories for comprehensive recent project list
- **Improved Search**: Fuzzy matching with Skim matcher
- **Better Project Detection**: More project types and smarter filtering
- **Makefile**: Professional build system with linting, testing, and packaging
- **Dev Tools**: `install-local` command for rapid development
- **Debug Tool**: `debug-zed-db` command for troubleshooting

### v1.0.0

- Initial release
- Project search with `zed` keyword
- Recent projects with `zedr` keyword
- Custom project directories via environment variables
- Project type detection and icons
- Alfred workflow configuration
