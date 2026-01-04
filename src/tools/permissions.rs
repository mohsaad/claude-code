use anyhow::Result;
use colored::Colorize;
use dialoguer::Input;

pub enum PermissionResponse {
    Yes,
    No,
    Trust,
}

/// Asks the user for permission to execute a tool
pub fn ask_permission(tool_name: &str, details: &str) -> Result<PermissionResponse> {
    println!("\n{}", format!("🔧 {} wants to execute:", tool_name).bright_yellow().bold());
    println!("{}", details);

    loop {
        let response: String = Input::new()
            .with_prompt("Allow? [y]es / [n]o / [t]rust")
            .default("n".to_string())
            .interact_text()?;

        match response.to_lowercase().trim() {
            "y" | "yes" => return Ok(PermissionResponse::Yes),
            "n" | "no" => return Ok(PermissionResponse::No),
            "t" | "trust" => return Ok(PermissionResponse::Trust),
            _ => {
                println!("{}", "Invalid response. Please enter y, n, or t".red());
                continue;
            }
        }
    }
}

// TODO: Implement trust mode persistence to file
