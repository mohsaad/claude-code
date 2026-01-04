use crate::api::types::Tool;
use serde_json::json;

/// Returns all available tools for the agent
pub fn get_tools() -> Vec<Tool> {
    vec![
        create_read_file_tool(),
        create_write_file_tool(),
        create_list_files_tool(),
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
