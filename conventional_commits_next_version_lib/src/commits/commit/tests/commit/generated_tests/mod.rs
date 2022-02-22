use super::*;

mod generation;

#[test]
fn test_major_title_commits() {
    for commit_messages in generation::generate_major_title_commits() {
        for commit_message in commit_messages {
            is_only_major_title_increment(&commit_message);
        }
    }
}

#[test]
fn test_major_footer_commits() {
    for commit_messages in generation::generate_major_footer_commits() {
        for commit_message in commit_messages {
            is_only_major_footer_increment(&commit_message);
        }
    }
}

#[test]
fn test_minor_commits() {
    for commit_messages in generation::generate_minor_commits() {
        for commit_message in commit_messages {
            is_minor_increment(&commit_message);
        }
    }
}

#[test]
fn test_patch_commits() {
    for commit_messages in generation::generate_patch_commits() {
        for commit_message in commit_messages {
            is_patch_increment(&commit_message);
        }
    }
}
