use semver::Version;

pub fn get_next_version_from_commits(
    commit_messages: Vec<String>,
    mut semantic_version: Version,
) -> Version {
    for (_i, commit_message) in commit_messages.iter().enumerate() {
        match get_commit_prefix(commit_message) {
            Some(commit_prefix) => match &*commit_prefix {
                "fix" => {
                    semantic_version.increment_patch();
                }
                "feat" => {
                    semantic_version.increment_minor();
                }
                "build" | "chore" | "ci" | "docs" | "style" | "refactor" | "perf" | "test" => {}
                _ => {
                    warn!("Unknown commit prefix '{}'.", commit_prefix);
                }
            },
            None => {
                warn!("Unable to parse commit.");
            }
        }
    }

    return semantic_version;
}

fn get_commit_prefix(commit_message: &str) -> Option<String> {
    let colon_split_string: Vec<&str> = commit_message.split(':').collect();

    if colon_split_string.len() > 0 {
        let prefix = colon_split_string[0].trim().to_string().to_lowercase();
        let bracket_split_string: Vec<&str> = prefix.split('(').collect();

        if bracket_split_string.len() > 0 {
            return Some(bracket_split_string[0].to_string());
        }

        return Some(prefix);
    }

    return None;
}

#[cfg(test)]
mod tests;
