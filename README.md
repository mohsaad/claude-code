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
- **edit_file** - Edits files by replacing specific text (requires permission: y/n/t)
- **list_files** - Lists files in a directory (no permission required)
- **execute_command** - Runs shell commands (requires permission: y/n/t)

### Features

- **Conversation History** - Maintains context across multiple prompts in REPL mode
- **Permission System** - Asks before executing any write or execute operations
- **Trust Mode** - Remember permissions for the session
- **System Message** - Includes guidance for Claude to act as a coding assistant
- **Colored Output** - Visual feedback for tool execution and results

### Permission System

When the agent wants to modify files or execute commands, it will ask for permission:
- **[y]es** - Allow this one time
- **[n]o** - Deny the operation
- **[t]rust** - Allow this tool for the rest of the session

### REPL Commands

- `clear` or `/clear` - Clear conversation history
- `exit` or `quit` - Exit the program

## Testing

### Automated Tests

The project includes comprehensive automated tests:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_edit_file
```

**Test Coverage:**
- **Unit Tests** (16 tests)
  - File operations (read, write, edit, list, validate)
  - API key configuration
  - Tool schema definitions
- **Integration Tests** (4 tests)
  - End-to-end file workflows
  - Directory operations
  - Security validation
  - Multi-line editing

### Manual Testing

A simple test web application is available in `test-app/index.html` for manual testing of the agent's file operations.

See [TESTING.md](TESTING.md) for detailed manual test scenarios and verification steps.

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
