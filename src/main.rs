use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use std::io::{self, Write};

mod api;
mod config;
mod fs;
mod tools;

use api::client::ClaudeClient;
use api::types::{ContentBlock, Message, MessageContent};
use tools::definitions::get_tools;
use tools::executor::ToolExecutor;

#[derive(Parser)]
#[command(name = "claude-code")]
#[command(about = "A Rust-based coding agent powered by Claude", long_about = None)]
struct Cli {
    /// Optional prompt to run directly
    #[arg(short, long)]
    prompt: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load API key from environment
    dotenv::dotenv().ok();
    let api_key = config::env::get_api_key()?;

    println!("{}", "Claude Code Agent".bright_cyan().bold());
    println!("{}", "Type your prompts or 'exit' to quit\n".dimmed());

    let client = ClaudeClient::new(api_key);
    let mut executor = ToolExecutor::new();

    if let Some(prompt) = cli.prompt {
        // Run single prompt mode
        run_prompt(&client, &mut executor, &prompt).await?;
    } else {
        // Run REPL mode
        run_repl(&client).await?;
    }

    Ok(())
}

async fn run_repl(client: &ClaudeClient) -> Result<()> {
    let mut executor = ToolExecutor::new();

    loop {
        print!("{} ", "›".bright_green().bold());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "exit" || input == "quit" {
            println!("{}", "Goodbye!".bright_cyan());
            break;
        }

        if let Err(e) = run_prompt(client, &mut executor, input).await {
            eprintln!("{} {}", "Error:".bright_red().bold(), e);
        }
    }

    Ok(())
}

async fn run_prompt(client: &ClaudeClient, executor: &mut ToolExecutor, prompt: &str) -> Result<()> {
    println!("\n{} {}", "You:".bright_blue().bold(), prompt);

    // Build initial message
    let mut messages = vec![Message {
        role: "user".to_string(),
        content: MessageContent::Text(prompt.to_string()),
    }];

    // Get available tools
    let tools = get_tools();

    // Conversation loop - handle tool use
    loop {
        let response = client.send_message(messages.clone(), Some(tools.clone())).await?;

        // Check if we have tool uses
        let mut has_tool_use = false;
        let mut text_response = String::new();
        let mut tool_results = Vec::new();

        for block in &response.content {
            match block {
                ContentBlock::Text { text } => {
                    text_response.push_str(text);
                }
                ContentBlock::ToolUse { id, name, input } => {
                    has_tool_use = true;
                    println!("\n{}", format!("🔧 Using tool: {}", name).bright_cyan());

                    // Execute the tool
                    match executor.execute_tool(name, input) {
                        Ok(result) => {
                            tool_results.push(ContentBlock::ToolResult {
                                tool_use_id: id.clone(),
                                content: result,
                            });
                        }
                        Err(e) => {
                            tool_results.push(ContentBlock::ToolResult {
                                tool_use_id: id.clone(),
                                content: format!("Error: {}", e),
                            });
                        }
                    }
                }
                _ => {}
            }
        }

        if !has_tool_use {
            // No tool use, display the text response and we're done
            if !text_response.is_empty() {
                println!("\n{} {}\n", "Claude:".bright_magenta().bold(), text_response);
            }
            break;
        }

        // If we have tool use, we need to send results back to Claude
        // Add assistant's response to conversation
        messages.push(Message {
            role: "assistant".to_string(),
            content: MessageContent::Blocks(response.content),
        });

        // Add tool results as user message
        messages.push(Message {
            role: "user".to_string(),
            content: MessageContent::Blocks(tool_results),
        });

        // Continue the loop to get Claude's next response
    }

    Ok(())
}
