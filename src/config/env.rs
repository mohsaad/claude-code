use anyhow::{Context, Result};
use std::env;

/// Gets the Claude API key from environment variables
pub fn get_api_key() -> Result<String> {
    env::var("ANTHROPIC_API_KEY")
        .or_else(|_| env::var("CLAUDE_API_KEY"))
        .context(
            "API key not found. Please set ANTHROPIC_API_KEY or CLAUDE_API_KEY environment variable.\n\
             You can create a .env file with:\n  ANTHROPIC_API_KEY=your_key_here"
        )
}
