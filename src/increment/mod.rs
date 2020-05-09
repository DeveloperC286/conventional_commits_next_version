use regex::Regex;
use semver::Version;

pub fn get_next_version_from_commits(
    commit_messages: Vec<String>,
    mut semantic_version: Version,
) -> Version {
    lazy_static! {
        static ref MINOR_INCREMENT_REGEX: Regex =
            Regex::new(r"(?i)^feat(\([[:word:]]*\))?:").unwrap();
        static ref PATCH_INCREMENT_REGEX: Regex =
            Regex::new(r"(?i)^fix(\([[:word:]]*\))?:").unwrap();
    }

    for (_i, commit_message) in commit_messages.iter().enumerate() {
        // Check for major increments.
        // TODO

        if MINOR_INCREMENT_REGEX.is_match(commit_message) {
            semantic_version.increment_minor();
        }

        if PATCH_INCREMENT_REGEX.is_match(commit_message) {
            semantic_version.increment_patch();
        }
    }

    return semantic_version;
}

#[cfg(test)]
mod tests;
