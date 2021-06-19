use super::*;

mod generated_tests;
mod major;
mod minor;
mod patch;

fn is_only_major_title_increment(commit_message: &str) {
    let commit = Commit {
        message: commit_message.to_string(),
    };

    assert!(
        commit.is_major_title_increment(),
        "{:?} does not match a major title Semantic Versioning increment commit message.",
        commit_message
    );
    assert!(
        !commit.is_major_footer_increment(),
        "{:?} matches a major footer Semantic Versioning increment commit message.",
        commit_message
    );
}

fn is_not_major_title_increment(commit_message: &str) {
    let commit = Commit {
        message: commit_message.to_string(),
    };

    assert!(
        !commit.is_major_title_increment(),
        "{:?} matches a major title Semantic Versioning increment commit message.",
        commit_message
    );
}

fn is_only_major_footer_increment(commit_message: &str) {
    let commit = Commit {
        message: commit_message.to_string(),
    };

    assert!(
        commit.is_major_footer_increment(),
        "{:?} does not match a major footer Semantic Versioning increment commit message.",
        commit_message
    );
    assert!(
        !commit.is_major_title_increment(),
        "{:?} matches a major title Semantic Versioning increment commit message.",
        commit_message
    );
}

fn is_major_title_and_footer_increment(commit_message: &str) {
    let commit = Commit {
        message: commit_message.to_string(),
    };

    assert!(
        commit.is_major_footer_increment(),
        "{:?} does not match a major footer Semantic Versioning increment commit message.",
        commit_message
    );
    assert!(
        commit.is_major_title_increment(),
        "{:?} does not match a major title Semantic Versioning increment commit message.",
        commit_message
    );
}

fn is_minor_increment(commit_message: &str) {
    let commit = Commit {
        message: commit_message.to_string(),
    };

    assert!(
        commit.is_minor_increment(),
        "{:?} does not match a minor Semantic Versioning increment commit message.",
        commit_message
    );
}

fn is_not_minor_increment(commit_message: &str) {
    let commit = Commit {
        message: commit_message.to_string(),
    };

    assert!(
        !commit.is_minor_increment(),
        "{:?} matches a minor Semantic Versioning increment commit message.",
        commit_message
    );
}

fn is_patch_increment(commit_message: &str) {
    let commit = Commit {
        message: commit_message.to_string(),
    };

    assert!(
        commit.is_patch_increment(),
        "{:?} does not match a patch Semantic Versioning increment commit message.",
        commit_message
    );
}

fn is_not_patch_increment(commit_message: &str) {
    let commit = Commit {
        message: commit_message.to_string(),
    };

    assert!(
        !commit.is_patch_increment(),
        "{:?} matches a patch Semantic Versioning increment commit message.",
        commit_message
    );
}
