use regex::Regex;
use semver::Version;

pub fn get_next_version_from_commits(
    commit_messages: Vec<String>,
    version: Version,
    batch_commits: bool,
) -> Version {
    if batch_commits {
        return get_next_version_from_commits_batch(commit_messages, version);
    } else {
        return get_next_version_from_commits_consecutive(commit_messages, version);
    }
}

fn get_next_version_from_commits_batch(
    commit_messages: Vec<String>,
    mut version: Version,
) -> Version {
    let mut major_commits_count = 0;
    let mut minor_commits_count = 0;
    let mut patch_commits_count = 0;

    for (_i, commit_message) in commit_messages.iter().enumerate() {
        if is_major_increment(commit_message) {
            trace!(
                "Incrementing major commits count because of commit {:?}.",
                commit_message
            );
            major_commits_count += 1;
        } else if is_minor_increment(commit_message) {
            trace!(
                "Incrementing minor commits count because of commit {:?}.",
                commit_message
            );
            minor_commits_count += 1;
        } else if is_patch_increment(commit_message) {
            trace!(
                "Incrementing patch commits count because of commit {:?}.",
                commit_message
            );
            patch_commits_count += 1;
        }
    }

    if major_commits_count > 0 {
        version.increment_major();
    } else if minor_commits_count > 0 {
        version.increment_minor();
    } else if patch_commits_count > 0 {
        version.increment_patch();
    }

    return version;
}

fn get_next_version_from_commits_consecutive(
    commit_messages: Vec<String>,
    mut version: Version,
) -> Version {
    for (_i, commit_message) in commit_messages.iter().enumerate() {
        if is_major_increment(commit_message) {
            trace!(
                "Incrementing major version because of commit {:?}.",
                commit_message
            );
            version.increment_major();
        } else if is_minor_increment(commit_message) {
            trace!(
                "Incrementing minor version because of commit {:?}.",
                commit_message
            );
            version.increment_minor();
        } else if is_patch_increment(commit_message) {
            trace!(
                "Incrementing patch version because of commit {:?}.",
                commit_message
            );
            version.increment_patch();
        }
    }

    return version;
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
