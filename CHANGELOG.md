# Changelog

## [Unreleased] - feature/agent-loop

### Added
- **Conversation History**: REPL now maintains conversation context across multiple prompts
  - History persists throughout the session
  - `/clear` command to reset conversation
- **edit_file Tool**: Precise file editing with search-and-replace functionality
  - Requires permission (y/n/t)
  - Shows exactly what will be changed
- **execute_command Tool**: Execute shell commands with safety checks
  - Requires permission (y/n/t)
  - Displays exit codes and output
  - Runs commands via `sh -c` for compatibility
- **System Message**: Added system prompt to guide Claude's behavior as a coding assistant
- **Enhanced Permission Formatting**: Improved visual feedback for permission requests
  - Color-coded diffs for edit operations
  - Clear command descriptions for execute operations

### Changed
- Updated README with new tools and features
- Updated CLAUDE.md with implementation status
- Enhanced tool executor with support for new operations

### Technical Details
- Added `edit_file` to `fs/operations.rs` with content validation
- Extended `tools/executor.rs` with edit and execute handlers
- Modified REPL loop to maintain conversation state
- Added system message to API requests

## [0.1.0] - Initial Release

### Added
- Basic CLI with REPL and single-prompt modes
- Claude API integration with tool use
- File operations: read_file, write_file, list_files
- Permission system with y/n/t prompts
- Trust mode for session-based permissions
- Path validation for security
- Colored terminal output
