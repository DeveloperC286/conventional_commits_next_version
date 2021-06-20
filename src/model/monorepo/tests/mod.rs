use super::*;

#[test]
fn test_files_within_monorepo() {
    // Given
    let monorepo = Monorepo {
        monorepo: Some("lib/".to_string()),
    };
    let files: HashSet<String> = {
        let mut files: HashSet<String> = HashSet::new();
        files.insert("lib/src/lib.rs".to_string());
        files
    };

    // When/Then
    assert!(monorepo.is_files_within(files));
}

#[test]
fn test_files_not_within_monorepo() {
    // Given
    let monorepo = Monorepo {
        monorepo: Some("lib/".to_string()),
    };
    let files: HashSet<String> = {
        let mut files: HashSet<String> = HashSet::new();
        files.insert("bin/src/main.rs".to_string());
        files
    };

    // When/Then
    assert!(!monorepo.is_files_within(files));
}
