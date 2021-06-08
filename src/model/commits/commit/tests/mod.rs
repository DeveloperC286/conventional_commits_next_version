use rstest::rstest;

use crate::model::commits::commit::Commit;

mod generated_tests;
mod major;
mod minor;
mod patch;

fn is_major_footer_increment(commit_message: &str) {
    let commit = Commit {
        message: commit_message.to_string(),
    };

    assert_eq!(
        true,
        commit.is_major_footer_increment(),
        "{:?} is not is_major_footer_increment()",
        commit_message
    );
}

fn is_major_title_increment(commit_message: &str) {
    let commit = Commit {
        message: commit_message.to_string(),
    };

    assert_eq!(
        true,
        commit.is_major_title_increment(),
        "{:?} is not is_major_title_increment()",
        commit_message
    );
}

fn is_not_major_title_increment(commit_message: &str) {
    let commit = Commit {
        message: commit_message.to_string(),
    };

    assert_eq!(
        false,
        commit.is_major_title_increment(),
        "{:?} is_major_title_increment()",
        commit_message
    );
}

fn is_minor_increment(commit_message: &str) {
    let commit = Commit {
        message: commit_message.to_string(),
    };

    assert_eq!(
        true,
        commit.is_minor_increment(),
        "{:?} is not is_minor_increment()",
        commit_message
    );
}
fn is_not_minor_increment(commit_message: &str) {
    let commit = Commit {
        message: commit_message.to_string(),
    };

    assert_eq!(
        false,
        commit.is_minor_increment(),
        "{:?} is not is_minor_increment()",
        commit_message
    );
}

fn is_patch_increment(commit_message: &str) {
    let commit = Commit {
        message: commit_message.to_string(),
    };

    assert_eq!(
        true,
        commit.is_patch_increment(),
        "{:?} is not is_patch_increment()",
        commit_message
    );
}
fn is_not_patch_increment(commit_message: &str) {
    let commit = Commit {
        message: commit_message.to_string(),
    };

    assert_eq!(
        false,
        commit.is_patch_increment(),
        "{:?} is_patch_increment()",
        commit_message
    );
}
