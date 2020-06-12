use regex::Regex;
use semver::Version;

lazy_static! {
    static ref SCOPE_REGEX: String = "([[:digit:]]|[[:alpha:]]|_|-| )*".to_string();
}

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
    if commit_messages
        .iter()
        .any(|commit_message| is_major_increment(commit_message))
    {
        version.increment_major();
        return version;
    }

    if commit_messages
        .iter()
        .any(|commit_message| is_minor_increment(commit_message))
    {
        version.increment_minor();
        return version;
    }

    if commit_messages
        .iter()
        .any(|commit_message| is_patch_increment(commit_message))
    {
        version.increment_patch();
        return version;
    }

    return version;
}

fn get_next_version_from_commits_consecutive(
    commit_messages: Vec<String>,
    mut version: Version,
) -> Version {
    commit_messages.iter().for_each(|commit_message| {
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
    });

    return version;
}

fn is_major_increment(commit_message: &str) -> bool {
    lazy_static! {
        static ref MAJOR_TITLE_INCREMENT_REGEX: Regex = Regex::new(
            format!(
                r"(?i)^({})(!(\({}\))?|(\({}\))?!):",
                &*SCOPE_REGEX, &*SCOPE_REGEX, &*SCOPE_REGEX
            )
            .as_str()
        )
        .unwrap();
        static ref MAJOR_FOOTER_INCREMENT_REGEX: Regex = Regex::new(
            format!(
                r"(?i)^({})(\({}\))?:(.)*(\n)*BREAKING CHANGE:",
                &*SCOPE_REGEX, &*SCOPE_REGEX
            )
            .as_str()
        )
        .unwrap();
    }

    return MAJOR_TITLE_INCREMENT_REGEX.is_match(commit_message)
        || MAJOR_FOOTER_INCREMENT_REGEX.is_match(commit_message);
}

fn is_minor_increment(commit_message: &str) -> bool {
    lazy_static! {
        static ref MINOR_INCREMENT_REGEX: Regex =
            Regex::new(format!(r"(?i)^feat(\({}\))?:", &*SCOPE_REGEX).as_str()).unwrap();
    }

    return MINOR_INCREMENT_REGEX.is_match(commit_message);
}

fn is_patch_increment(commit_message: &str) -> bool {
    lazy_static! {
        static ref PATCH_INCREMENT_REGEX: Regex =
            Regex::new(format!(r"(?i)^fix(\({}\))?:", &*SCOPE_REGEX).as_str()).unwrap();
    }

    return PATCH_INCREMENT_REGEX.is_match(commit_message);
}

#[cfg(test)]
mod tests;
