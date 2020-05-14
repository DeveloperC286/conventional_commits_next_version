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
    lazy_static! {
        static ref MAJOR_TITLE_INCREMENT_REGEX: Regex =
            Regex::new(r"(?i)^([[:word:]]*)(!(\([[:word:]]*\))?|(\([[:word:]]*\))?!):").unwrap();
        static ref MAJOR_FOOTER_INCREMENT_REGEX: Regex =
            Regex::new(r"(?i)^([[:word:]]*)(\([[:word:]]*\))?:(.)*(\n)*BREAKING CHANGE:").unwrap();
        static ref MINOR_INCREMENT_REGEX: Regex =
            Regex::new(r"(?i)^feat(\([[:word:]]*\))?:").unwrap();
        static ref PATCH_INCREMENT_REGEX: Regex =
            Regex::new(r"(?i)^fix(\([[:word:]]*\))?:").unwrap();
    }

    for (_i, commit_message) in commit_messages.iter().enumerate() {
        if MAJOR_TITLE_INCREMENT_REGEX.is_match(commit_message)
            || MAJOR_FOOTER_INCREMENT_REGEX.is_match(commit_message)
        {
            trace!(
                "Incrementing major version because of commit {:?}.",
                commit_message
            );
            semantic_version.increment_major();
        } else if MINOR_INCREMENT_REGEX.is_match(commit_message) {
            trace!(
                "Incrementing minor version because of commit {:?}.",
                commit_message
            );
            semantic_version.increment_minor();
        } else if PATCH_INCREMENT_REGEX.is_match(commit_message) {
            trace!(
                "Incrementing patch version because of commit {:?}.",
                commit_message
            );
            semantic_version.increment_patch();
        }
    }

    return semantic_version;
}

#[cfg(test)]
mod tests;
