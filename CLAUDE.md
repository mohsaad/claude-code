# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based coding agent that interfaces with the Claude API. The agent can take prompts, call Claude's API, and execute system-level tools for file operations with user permission.

## Build and Development Commands

**Build the project:**
```bash
cargo build
```

**Build release version:**
```bash
cargo build --release
```

**Run in development:**
```bash
cargo run
```

**Run with a single prompt:**
```bash
cargo run -- -p "your prompt here"
```

**Run tests:**
```bash
cargo test
```

**Check for issues:**
```bash
cargo clippy
```

**Format code:**
```bash
cargo fmt
```

## Architecture

### Core Modules

**`src/main.rs`** - Entry point with CLI and REPL loop
- Implements command-line interface using `clap`
- Provides REPL mode for interactive prompts
- Supports single-prompt execution with `-p` flag

**`src/api/`** - Claude API integration
- `client.rs` - HTTP client for Claude API using `reqwest`
- `types.rs` - Request/response types and content blocks
- Currently uses `claude-3-5-sonnet-20241022` model with 4096 max tokens

**`src/config/`** - Configuration management
- `env.rs` - API key loading from environment variables
- Supports both `ANTHROPIC_API_KEY` and `CLAUDE_API_KEY`
- Uses `dotenv` for `.env` file support

**`src/fs/`** - File system operations
- `operations.rs` - Safe file reading, writing, and listing
- Includes path validation to prevent directory traversal

**`src/tools/`** - Tool execution framework
- `definitions.rs` - Tool schema definitions (read_file, write_file, edit_file, list_files, execute_command)
- `executor.rs` - Tool execution logic with permission checks and command execution
- `permissions.rs` - y/n/t permission prompts using `dialoguer`

### Key Design Patterns

**Async Runtime**: Uses Tokio for async operations, particularly for API calls

**Error Handling**: Uses `anyhow::Result` for error propagation with context

**Permission Model**: Designed to ask before executing file-modifying tools with y/n/t (yes/no/trust) options

**API Communication**: Structured message format matching Claude's API schema with support for tool use

## Testing

A test web application is available in `test-app/index.html` for validating file operations.

See [TESTING.md](TESTING.md) for comprehensive test scenarios:
- Reading files
- Writing files with permission system
- Listing directories
- Trust mode verification

## Configuration

Create a `.env` file (copy from `.env.example`):
```
ANTHROPIC_API_KEY=your_key_here
```

## Current Implementation Status

**Completed:**
- ✅ Basic CLI with REPL mode
- ✅ Claude API client with tool use support and system messages
- ✅ API key configuration
- ✅ File operations module (read, write, edit, list, validate)
- ✅ Tool definitions (read_file, write_file, edit_file, list_files, execute_command)
- ✅ Tool executor with permission system
- ✅ Full conversation loop with tool execution
- ✅ Trust mode (session-based)
- ✅ Conversation history (maintained across REPL prompts)
- ✅ Command execution with safety checks

**To Implement:**
- ⏳ Trust mode persistence to file (currently only lasts for session)
- ⏳ Conversation history persistence across sessions
- ⏳ Code syntax highlighting in output
- ⏳ Streaming API responses for real-time feedback

## Notes for Development

- The project follows standard Rust conventions with the 2021 edition
- All async code uses the Tokio runtime
- Path validation is critical - always use `fs::operations::validate_path` before file operations
- Permission prompts should show clear details about what will be modified
- The agent is designed to show generated code before writing it to files
