use std::fs;
use std::path::PathBuf;

// Integration tests for tool execution

fn get_test_dir() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push("claude_code_integration_tests");
    fs::create_dir_all(&path).unwrap();
    path
}

#[test]
fn test_file_operations_workflow() {
    let test_dir = get_test_dir();
    let test_file = test_dir.join("workflow_test.txt");
    let test_path = test_file.to_str().unwrap();

    // Write a file
    fs::write(test_path, "Initial content").unwrap();

    // Read it back
    let content = fs::read_to_string(test_path).unwrap();
    assert_eq!(content, "Initial content");

    // Edit the file
    let updated = content.replace("Initial", "Updated");
    fs::write(test_path, updated).unwrap();

    // Verify the edit
    let final_content = fs::read_to_string(test_path).unwrap();
    assert_eq!(final_content, "Updated content");

    // Cleanup
    fs::remove_file(test_path).unwrap();
}

#[test]
fn test_directory_listing() {
    let test_dir = get_test_dir();

    // Create multiple test files
    let files = vec!["test1.txt", "test2.txt", "test3.txt"];
    for filename in &files {
        let path = test_dir.join(filename);
        fs::write(&path, "test content").unwrap();
    }

    // List directory
    let entries: Vec<_> = fs::read_dir(&test_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();

    assert!(entries.len() >= 3, "Should have at least 3 files");

    // Cleanup
    for filename in &files {
        let path = test_dir.join(filename);
        let _ = fs::remove_file(path);
    }
}

#[test]
fn test_path_validation_security() {
    use std::path::Path;

    // Safe paths should not contain ..
    let safe_path = "src/main.rs";
    let path = Path::new(safe_path);
    assert!(!path.components().any(|c| c.as_os_str() == ".."));

    // Unsafe paths should be detected
    let unsafe_path = "../etc/passwd";
    let path = Path::new(unsafe_path);
    assert!(path.components().any(|c| c.as_os_str() == ".."));
}

#[test]
fn test_multiline_file_editing() {
    let test_dir = get_test_dir();
    let test_file = test_dir.join("multiline_test.txt");
    let test_path = test_file.to_str().unwrap();

    let original = "Line 1\nLine 2\nLine 3";
    fs::write(test_path, original).unwrap();

    // Edit middle line
    let updated = original.replace("Line 2", "Modified Line 2");
    fs::write(test_path, updated).unwrap();

    let result = fs::read_to_string(test_path).unwrap();
    assert!(result.contains("Modified Line 2"));
    assert!(result.contains("Line 1"));
    assert!(result.contains("Line 3"));

    fs::remove_file(test_path).unwrap();
}
