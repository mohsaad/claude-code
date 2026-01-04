# Claude Code

A Rust-based coding agent powered by Claude AI.

## Setup

1. Install Rust (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone and set up the project:
   ```bash
   git clone <repository-url>
   cd claude-code
   ```

3. Create a `.env` file with your Anthropic API key:
   ```bash
   cp .env.example .env
   # Edit .env and add your API key
   ```

4. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

Run in REPL mode:
```bash
cargo run
```

Run with a single prompt:
```bash
cargo run -- -p "Read the file test-app/index.html"
```

### Available Tools

The agent has access to the following tools:

- **read_file** - Reads file contents (no permission required)
- **write_file** - Writes content to files (requires permission: y/n/t)
- **list_files** - Lists files in a directory (no permission required)

### Permission System

When the agent wants to modify files, it will ask for permission:
- **[y]es** - Allow this one time
- **[n]o** - Deny the operation
- **[t]rust** - Allow this tool for the rest of the session

## Testing

A simple test web application is available in `test-app/index.html` for testing the agent's file operations.

See [TESTING.md](TESTING.md) for detailed test scenarios and verification steps.

## Project Structure

- `src/main.rs` - CLI entry point and REPL loop
- `src/api/` - Claude API client and types
- `src/config/` - Configuration and API key management
- `src/fs/` - File system operations
- `src/tools/` - Tool execution and permission handling
- `test-app/` - Simple web app for testing

## Development

Build and run:
```bash
cargo run
```

Run tests:
```bash
cargo test
```

Check for issues:
```bash
cargo clippy
```
