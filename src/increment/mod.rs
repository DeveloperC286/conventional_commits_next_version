use regex::Regex;
use semver::Version;

pub fn get_next_version_from_commits(
    commit_messages: Vec<String>,
    semantic_version: Version,
    batch_commits: bool,
) -> Version {
    return get_next_version_from_commits_consecutive(commit_messages, semantic_version);
}

fn get_next_version_from_commits_consecutive(
    commit_messages: Vec<String>,
    mut semantic_version: Version,
) -> Version {
    for (_i, commit_message) in commit_messages.iter().enumerate() {
        if is_major_increment(commit_message) {
            trace!(
                "Incrementing major version because of commit {:?}.",
                commit_message
            );
            semantic_version.increment_major();
        } else if is_minor_increment(commit_message) {
            trace!(
                "Incrementing minor version because of commit {:?}.",
                commit_message
            );
            semantic_version.increment_minor();
        } else if is_patch_increment(commit_message) {
            trace!(
                "Incrementing patch version because of commit {:?}.",
                commit_message
            );
            semantic_version.increment_patch();
        }
    }

    return semantic_version;
}

fn is_major_increment(commit_message: &str) -> bool {
    lazy_static! {
        static ref MAJOR_TITLE_INCREMENT_REGEX: Regex =
            Regex::new(r"(?i)^([[:word:]]*)(!(\([[:word:]]*\))?|(\([[:word:]]*\))?!):").unwrap();
        static ref MAJOR_FOOTER_INCREMENT_REGEX: Regex =
            Regex::new(r"(?i)^([[:word:]]*)(\([[:word:]]*\))?:(.)*(\n)*BREAKING CHANGE:").unwrap();
    }

    return MAJOR_TITLE_INCREMENT_REGEX.is_match(commit_message)
        || MAJOR_FOOTER_INCREMENT_REGEX.is_match(commit_message);
}

fn is_minor_increment(commit_message: &str) -> bool {
    lazy_static! {
        static ref MINOR_INCREMENT_REGEX: Regex =
            Regex::new(r"(?i)^feat(\([[:word:]]*\))?:").unwrap();
    }

    return MINOR_INCREMENT_REGEX.is_match(commit_message);
}

fn is_patch_increment(commit_message: &str) -> bool {
    lazy_static! {
        static ref PATCH_INCREMENT_REGEX: Regex =
            Regex::new(r"(?i)^fix(\([[:word:]]*\))?:").unwrap();
    }

    return PATCH_INCREMENT_REGEX.is_match(commit_message);
}

#[cfg(test)]
mod tests;
