# Implementation Approach for Claude Code

## Architecture

### Core Components

**Agent Loop (Main Driver)**
- Command-line interface using `clap` for argument parsing
- REPL loop for interactive prompts
- State management for conversation history

**Claude API Client**
- Use `reqwest` for HTTP requests to Claude API
- Implement streaming responses for real-time output
- Handle token counting and context management
- Support for tool use (Claude's function calling feature)

**Tool System**
- Define tools as Claude API tool definitions (read_file, write_file, edit_file, etc.)
- Tool executor that maps Claude's tool calls to actual Rust system operations
- Permission system for the y/n/t approval flow

**File Operations Module**
- Safe file reading/writing with proper error handling
- Diff generation before writing (show what will change)
- Sandboxing/safety checks

### Project Structure

```
claude-code/
├── src/
│   ├── main.rs              # CLI entry point, REPL loop
│   ├── api/
│   │   ├── client.rs        # Claude API client
│   │   └── types.rs         # API request/response types
│   ├── tools/
│   │   ├── mod.rs           # Tool definitions
│   │   ├── executor.rs      # Tool execution logic
│   │   └── permissions.rs   # y/n/t permission handler
│   ├── fs/
│   │   └── operations.rs    # File system operations
│   └── config/
│       └── env.rs           # API key management
├── test-app/                # Simple web app for testing
│   └── index.html
└── Cargo.toml
```

### Key Dependencies

```toml
[dependencies]
clap = "4.0"           # CLI argument parsing
reqwest = "0.11"       # HTTP client for Claude API
tokio = "1.0"          # Async runtime
serde = "1.0"          # Serialization
serde_json = "1.0"     # JSON handling
anyhow = "1.0"         # Error handling
dialoguer = "0.11"     # Interactive prompts (y/n/t)
colored = "2.0"        # Terminal colors
```

### Implementation Phases

**Phase 1: Basic Setup**
- Set up Rust project with Cargo
- Implement API key storage (use `.env` file or system keychain)
- Basic Claude API connection with simple prompt/response

**Phase 2: Tool System**
- Implement tool definitions matching Claude's tool use format
- Create read_file, write_file, list_files tools
- Build permission system with y/n/t prompts

**Phase 3: Agent Loop**
- Implement conversation loop with history
- Handle tool calls from Claude
- Display generated code before writing
- Execute approved tool calls

**Phase 4: Polish**
- Add syntax highlighting for code output
- Improve error messages
- Add conversation saving/loading
- Implement "trust" mode persistence

### Critical Design Decisions

**API Key Storage**: Use `keyring` crate for secure storage rather than just environment variables (more secure than plain .env)

**Permission Model**:
- Store "trusted" tools in a config file (`~/.claude-code/trust.json`)
- Allow per-tool trust or per-directory trust
- Always show diffs before writes, even in trust mode

**Streaming**: Implement streaming responses so users see Claude's thinking in real-time

**Safety**:
- Validate file paths (prevent writes outside working directory)
- Dry-run mode for testing
- Backup files before modification

### Testing Approach

For the test web app:
```html
<!-- test-app/index.html -->
<!DOCTYPE html>
<html>
<head><title>Test App</title></head>
<body><h1>Hello World</h1></body>
</html>
```

Test scenarios:
- "Read the HTML file"
- "Change 'Hello World' to 'Hello Claude'"
- "Add a CSS file with styling"
- Verify permission prompts work
- Verify trust mode works

### Initial Setup Steps

```bash
# Initialize project
cargo new claude-code
cd claude-code

# Add dependencies to Cargo.toml
# Create basic main.rs with CLI
# Implement simple Claude API call
# Build from there
```
