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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_api_key_anthropic() {
        env::set_var("ANTHROPIC_API_KEY", "test_key_123");
        env::remove_var("CLAUDE_API_KEY");

        let result = get_api_key();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_key_123");

        env::remove_var("ANTHROPIC_API_KEY");
    }

    #[test]
    fn test_get_api_key_claude() {
        env::remove_var("ANTHROPIC_API_KEY");
        env::set_var("CLAUDE_API_KEY", "test_key_456");

        let result = get_api_key();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_key_456");

        env::remove_var("CLAUDE_API_KEY");
    }

    #[test]
    fn test_get_api_key_prefers_anthropic() {
        env::set_var("ANTHROPIC_API_KEY", "anthropic_key");
        env::set_var("CLAUDE_API_KEY", "claude_key");

        let result = get_api_key();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "anthropic_key");

        env::remove_var("ANTHROPIC_API_KEY");
        env::remove_var("CLAUDE_API_KEY");
    }

    #[test]
    fn test_get_api_key_missing() {
        env::remove_var("ANTHROPIC_API_KEY");
        env::remove_var("CLAUDE_API_KEY");

        let result = get_api_key();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("API key not found"));
    }
}
