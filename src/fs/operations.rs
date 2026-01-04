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
