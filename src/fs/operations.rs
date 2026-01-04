use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn read_file(path: &str) -> Result<String> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

pub fn write_file(path: &str, content: &str) -> Result<()> {
    fs::write(path, content)?;
    Ok(())
}

pub fn edit_file(path: &str, old_text: &str, new_text: &str) -> Result<()> {
    let content = read_file(path)?;

    if !content.contains(old_text) {
        anyhow::bail!("Text to replace not found in file");
    }

    let new_content = content.replace(old_text, new_text);
    write_file(path, &new_content)?;

    Ok(())
}

pub fn list_files(dir: &str) -> Result<Vec<String>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(path_str) = path.to_str() {
            files.push(path_str.to_string());
        }
    }

    Ok(files)
}

pub fn validate_path(path: &str) -> Result<()> {
    let path = Path::new(path);

    // Prevent directory traversal attacks
    if path.components().any(|c| c.as_os_str() == "..") {
        anyhow::bail!("Path contains '..' which is not allowed");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn get_test_dir() -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push("claude_code_tests");
        fs::create_dir_all(&path).unwrap();
        path
    }

    fn cleanup_test_file(path: &str) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_write_and_read_file() {
        let test_dir = get_test_dir();
        let test_file = test_dir.join("test_write_read.txt");
        let test_path = test_file.to_str().unwrap();

        let content = "Hello, World!";
        write_file(test_path, content).unwrap();

        let read_content = read_file(test_path).unwrap();
        assert_eq!(read_content, content);

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_edit_file() {
        let test_dir = get_test_dir();
        let test_file = test_dir.join("test_edit.txt");
        let test_path = test_file.to_str().unwrap();

        write_file(test_path, "Hello, World!").unwrap();

        edit_file(test_path, "World", "Rust").unwrap();

        let content = read_file(test_path).unwrap();
        assert_eq!(content, "Hello, Rust!");

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_edit_file_text_not_found() {
        let test_dir = get_test_dir();
        let test_file = test_dir.join("test_edit_not_found.txt");
        let test_path = test_file.to_str().unwrap();

        write_file(test_path, "Hello, World!").unwrap();

        let result = edit_file(test_path, "Python", "Rust");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));

        cleanup_test_file(test_path);
    }

    #[test]
    fn test_list_files() {
        let test_dir = get_test_dir();
        let test_dir_str = test_dir.to_str().unwrap();

        // Create some test files
        let file1 = test_dir.join("file1.txt");
        let file2 = test_dir.join("file2.txt");
        write_file(file1.to_str().unwrap(), "content1").unwrap();
        write_file(file2.to_str().unwrap(), "content2").unwrap();

        let files = list_files(test_dir_str).unwrap();
        assert!(files.len() >= 2);

        cleanup_test_file(file1.to_str().unwrap());
        cleanup_test_file(file2.to_str().unwrap());
    }

    #[test]
    fn test_validate_path_valid() {
        assert!(validate_path("src/main.rs").is_ok());
        assert!(validate_path("test-app/index.html").is_ok());
        assert!(validate_path("./file.txt").is_ok());
    }

    #[test]
    fn test_validate_path_invalid() {
        assert!(validate_path("../etc/passwd").is_err());
        assert!(validate_path("foo/../bar").is_err());
        assert!(validate_path("../../secret").is_err());
    }

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_file("/nonexistent/path/to/file.txt");
        assert!(result.is_err());
    }
}
