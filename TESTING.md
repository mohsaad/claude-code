# Testing Guide

This guide provides test scenarios to verify the Claude Code agent's functionality.

## Setup

1. Ensure you have an API key configured:
   ```bash
   cp .env.example .env
   # Edit .env and add your ANTHROPIC_API_KEY
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Test Scenarios

### Test 1: Read File
Test that the agent can read existing files.

**Prompt:**
```
Read the file test-app/index.html
```

**Expected behavior:**
- Agent uses `read_file` tool
- Displays the HTML content
- No permission prompt (read-only operation)

### Test 2: List Files
Test directory listing functionality.

**Prompt:**
```
List all files in the test-app directory
```

**Expected behavior:**
- Agent uses `list_files` tool
- Shows all files in test-app/
- No permission prompt (read-only operation)

### Test 3: Write File (Permission)
Test the permission system for write operations.

**Prompt:**
```
Change the text in test-app/index.html from "Hello World" to "Hello Claude"
```

**Expected behavior:**
- Agent reads the file first
- Shows the new content it will write
- Asks for permission with y/n/t options
- If you answer 'y', writes once
- If you answer 't', writes and adds to trusted list
- If you answer 'n', operation is cancelled

### Test 4: Trust Mode
After trusting write_file in Test 3, try another write operation.

**Prompt:**
```
Add a paragraph with the text "This is a test" to the HTML file
```

**Expected behavior:**
- Agent writes without asking permission
- Shows "✓ 'write_file' is now trusted" (already trusted)
- Completes operation automatically

### Test 5: Multi-Step Operation
Test that the agent can chain multiple tools.

**Prompt:**
```
Create a new file called test-app/styles.css with basic styling for the h1 tag
```

**Expected behavior:**
- Agent may use read_file to check if file exists
- Uses write_file to create the CSS
- Asks for permission before writing
- Creates valid CSS content

## Running Tests

### Interactive Mode (REPL)
```bash
cargo run
```
Then type each prompt from the test scenarios above.

### Single Prompt Mode
```bash
cargo run -- -p "Read the file test-app/index.html"
```

## Verification

After running tests, verify:

1. ✅ Files are read correctly
2. ✅ Directory listings are accurate
3. ✅ Permission prompts appear for write operations
4. ✅ Trust mode persists during the session
5. ✅ File modifications are correct
6. ✅ Error messages are clear and helpful

## Common Issues

**Issue:** API key not found
- **Solution:** Ensure `.env` file exists with `ANTHROPIC_API_KEY=your_key`

**Issue:** Permission denied on file operations
- **Solution:** Check file/directory permissions with `ls -la`

**Issue:** Path validation errors
- **Solution:** Ensure you're using relative paths without `..`
