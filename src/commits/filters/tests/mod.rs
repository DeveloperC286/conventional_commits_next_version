use rstest::rstest;

use super::*;

macro_rules! filters {
    ($filters:expr) => {
        Filters {
            commits_must_effect: $filters.iter().map(|filters| filters.to_string()).collect(),
        }
    };
}

macro_rules! files {
    ($files:expr) => {
        $files.iter().map(|file| file.to_string()).collect()
    };
}

#[rstest]
#[case(&["lib/"], &["lib/src/lib.rs"])]
#[case(&["bin/"], &["bin/src/main.rs"])]
fn test_file_within_a_filters(#[case] filters: &[&str], #[case] files: &[&str]) {
    // Given
    let filters = filters!(filters);
    let files: HashSet<String> = files!(files);

    // When/Then
    assert!(filters.is_files_within(files));
}

#[rstest]
#[case(&["lib/"], &["bin/src/main.rs"])]
#[case(&["bin/"], &["README.md"])]
fn test_file_not_within_a_filters(#[case] filters: &[&str], #[case] files: &[&str]) {
    // Given
    let filters = filters!(filters);
    let files: HashSet<String> = files!(files);

    // When/Then
    assert!(!filters.is_files_within(files));
}

#[rstest]
#[case(&["lib/", "bin/"], &["bin/src/main.rs"])]
#[case(&["bin/", "ops/"], &["ops/main.tf"])]
fn test_file_within_filters(#[case] filters: &[&str], #[case] files: &[&str]) {
    // Given
    let filters = filters!(filters);
    let files: HashSet<String> = files!(files);

    // When/Then
    assert!(filters.is_files_within(files));
}

#[rstest]
#[case(&["lib/", "lib2/"], &["bin/src/main.rs"])]
#[case(&["lib/", "bin/"], &["ops/main.tf"])]
fn test_file_not_within_filters(#[case] filters: &[&str], #[case] files: &[&str]) {
    // Given
    let filters = filters!(filters);
    let files: HashSet<String> = files!(files);

    // When/Then
    assert!(!filters.is_files_within(files));
}

#[rstest]
#[case(&["lib/"], &["lib/src/lib.rs", "lib/src/model/mod.rs", "ops/main.tf", "README.md"])]
#[case(&["bin/"], &[".gitignore", "README.md", "bin/src/main.rs"])]
fn test_files_within_a_filters(#[case] filters: &[&str], #[case] files: &[&str]) {
    // Given
    let filters = filters!(filters);
    let files: HashSet<String> = files!(files);

    // When/Then
    assert!(filters.is_files_within(files));
}

#[rstest]
#[case(&["lib/"], &["build.sh", "bin/src/main.rs", "bin/src/model/mod.rs"])]
#[case(&["bin/"], &["README.md", "lib/build.sh", "lib/src/lib.rs"])]
fn test_files_not_within_a_filters(#[case] filters: &[&str], #[case] files: &[&str]) {
    // Given
    let filters = filters!(filters);
    let files: HashSet<String> = files!(files);

    // When/Then
    assert!(!filters.is_files_within(files));
}

#[rstest]
#[case(&["lib/", "bin/"], &["README.md", "lib/src/lib.rs", "bin/src/main.rs"])]
#[case(&["bin/", "ops/"], &["build.sh", "bin/README.md", "ops/main.tf"])]
fn test_files_within_filters(#[case] filters: &[&str], #[case] files: &[&str]) {
    // Given
    let filters = filters!(filters);
    let files: HashSet<String> = files!(files);

    // When/Then
    assert!(filters.is_files_within(files));
}

#[rstest]
#[case(&["lib/", "ops/"], &[".gitignore", "bin/README.md", "bin/src/main.rs"])]
#[case(&["lib/", "bin/"], &["build.sh", "README.md", "ops/main.tf"])]
fn test_files_not_within_filters(#[case] filters: &[&str], #[case] files: &[&str]) {
    // Given
    let filters = filters!(filters);
    let files: HashSet<String> = files!(files);

    // When/Then
    assert!(!filters.is_files_within(files));
}
