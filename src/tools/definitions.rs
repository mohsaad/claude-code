use crate::api::types::Tool;
use serde_json::json;

/// Returns all available tools for the agent
pub fn get_tools() -> Vec<Tool> {
    vec![
        create_read_file_tool(),
        create_write_file_tool(),
        create_edit_file_tool(),
        create_list_files_tool(),
        create_execute_command_tool(),
    ]
}

fn create_read_file_tool() -> Tool {
    Tool {
        name: "read_file".to_string(),
        description: "Reads the contents of a file at the specified path. Use this to understand existing code or files.".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The path to the file to read"
                }
            },
            "required": ["path"]
        }),
    }
}

fn create_write_file_tool() -> Tool {
    Tool {
        name: "write_file".to_string(),
        description: "Writes content to a file at the specified path. This will overwrite existing files. Always show the user what will be written before calling this tool.".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The path to the file to write"
                },
                "content": {
                    "type": "string",
                    "description": "The content to write to the file"
                }
            },
            "required": ["path", "content"]
        }),
    }
}

fn create_edit_file_tool() -> Tool {
    Tool {
        name: "edit_file".to_string(),
        description: "Edits a file by replacing specific text. More precise than write_file for making small changes. Provide the exact text to find and replace.".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "The path to the file to edit"
                },
                "old_text": {
                    "type": "string",
                    "description": "The exact text to find and replace (must match exactly including whitespace)"
                },
                "new_text": {
                    "type": "string",
                    "description": "The new text to replace the old text with"
                }
            },
            "required": ["path", "old_text", "new_text"]
        }),
    }
}

fn create_list_files_tool() -> Tool {
    Tool {
        name: "list_files".to_string(),
        description: "Lists all files in a directory. Use this to explore the project structure.".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "directory": {
                    "type": "string",
                    "description": "The directory path to list files from"
                }
            },
            "required": ["directory"]
        }),
    }
}

fn create_execute_command_tool() -> Tool {
    Tool {
        name: "execute_command".to_string(),
        description: "Executes a shell command and returns the output. Use this to run build commands, tests, or other operations. Commands will require user permission.".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "The shell command to execute"
                },
                "description": {
                    "type": "string",
                    "description": "A brief description of what this command does"
                }
            },
            "required": ["command"]
        }),
    }
}
