use regex::Regex;
use semver::Version;

pub fn get_next_version_from_commits(
    commit_messages: Vec<String>,
    mut semantic_version: Version,
) -> Version {
    lazy_static! {
        static ref MAJOR_INCREMENT_REGEX: Regex =
            Regex::new(r"(?i)^([[:word:]]*)(!(\([[:word:]]*\))?|(\([[:word:]]*\))?!):").unwrap();
        static ref MINOR_INCREMENT_REGEX: Regex =
            Regex::new(r"(?i)^feat(\([[:word:]]*\))?:").unwrap();
        static ref PATCH_INCREMENT_REGEX: Regex =
            Regex::new(r"(?i)^fix(\([[:word:]]*\))?:").unwrap();
    }

    for (_i, commit_message) in commit_messages.iter().enumerate() {
        if MAJOR_INCREMENT_REGEX.is_match(commit_message) {
            trace!(
                "Incrementing major version because of commit {:?}.",
                commit_message
            );
            semantic_version.increment_major();
        }

        if MINOR_INCREMENT_REGEX.is_match(commit_message) {
            trace!(
                "Incrementing minor version because of commit {:?}.",
                commit_message
            );
            semantic_version.increment_minor();
        }

        if PATCH_INCREMENT_REGEX.is_match(commit_message) {
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
