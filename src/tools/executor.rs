use anyhow::{Context, Result};
use colored::Colorize;
use serde_json::Value;

use crate::fs::operations;
use super::permissions::{ask_permission, PermissionResponse};

pub struct ToolExecutor {
    // Track trusted tools
    trusted_tools: Vec<String>,
}

impl ToolExecutor {
    pub fn new() -> Self {
        Self {
            trusted_tools: Vec::new(),
        }
    }

    /// Execute a tool based on its name and input parameters
    pub fn execute_tool(&mut self, tool_name: &str, tool_input: &Value) -> Result<String> {
        // Check if tool requires permission
        if self.needs_permission(tool_name) {
            let details = self.format_tool_details(tool_name, tool_input);

            match ask_permission(tool_name, &details)? {
                PermissionResponse::Yes => {
                    // Execute once
                }
                PermissionResponse::No => {
                    return Ok("Permission denied by user".to_string());
                }
                PermissionResponse::Trust => {
                    // Add to trusted list
                    self.trusted_tools.push(tool_name.to_string());
                    println!("{}", format!("  ✓ '{}' is now trusted", tool_name).green());
                }
            }
        }

        // Execute the tool
        match tool_name {
            "read_file" => self.execute_read_file(tool_input),
            "write_file" => self.execute_write_file(tool_input),
            "list_files" => self.execute_list_files(tool_input),
            _ => anyhow::bail!("Unknown tool: {}", tool_name),
        }
    }

    fn needs_permission(&self, tool_name: &str) -> bool {
        // write_file always needs permission unless trusted
        // read_file and list_files don't need permission (read-only)
        match tool_name {
            "write_file" => !self.trusted_tools.contains(&tool_name.to_string()),
            _ => false,
        }
    }

    fn format_tool_details(&self, tool_name: &str, tool_input: &Value) -> String {
        match tool_name {
            "write_file" => {
                let path = tool_input.get("path")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                let content = tool_input.get("content")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                format!("Write to file: {}\n\nContent preview:\n{}\n",
                    path.bright_yellow(),
                    if content.len() > 200 {
                        format!("{}...\n({} total characters)", &content[..200], content.len())
                    } else {
                        content.to_string()
                    }
                )
            }
            _ => format!("Execute {}: {:?}", tool_name, tool_input),
        }
    }

    fn execute_read_file(&self, input: &Value) -> Result<String> {
        let path = input.get("path")
            .and_then(|v| v.as_str())
            .context("Missing 'path' parameter")?;

        operations::validate_path(path)?;
        let content = operations::read_file(path)
            .context(format!("Failed to read file: {}", path))?;

        println!("{}", format!("  ✓ Read {} ({} bytes)", path, content.len()).green());
        Ok(content)
    }

    fn execute_write_file(&self, input: &Value) -> Result<String> {
        let path = input.get("path")
            .and_then(|v| v.as_str())
            .context("Missing 'path' parameter")?;

        let content = input.get("content")
            .and_then(|v| v.as_str())
            .context("Missing 'content' parameter")?;

        operations::validate_path(path)?;
        operations::write_file(path, content)
            .context(format!("Failed to write file: {}", path))?;

        println!("{}", format!("  ✓ Wrote {} ({} bytes)", path, content.len()).green());
        Ok(format!("Successfully wrote {} bytes to {}", content.len(), path))
    }

    fn execute_list_files(&self, input: &Value) -> Result<String> {
        let directory = input.get("directory")
            .and_then(|v| v.as_str())
            .context("Missing 'directory' parameter")?;

        operations::validate_path(directory)?;
        let files = operations::list_files(directory)
            .context(format!("Failed to list directory: {}", directory))?;

        println!("{}", format!("  ✓ Listed {} files in {}", files.len(), directory).green());
        Ok(files.join("\n"))
    }
}
