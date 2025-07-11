use regex::Regex;

const OPTIONAL_PRECEDING_WHITESPACE: &str = "^([[:space:]])*";
const ANY_CHARACTER_REGEX: &str = "([[:digit:]]|[[:alpha:]])+";
const OPTIONAL_ANY_REGEX: &str = "([[:digit:]]|[[:alpha:]]|_|-|[[:space:]])*";

lazy_static! {
    static ref OPTIONAL_SCOPE_REGEX: String = format!(r"(\({OPTIONAL_ANY_REGEX}\))?");
    static ref MAJOR_TITLE_INCREMENT_REGEX: Regex = Regex::new(
        format!(
            r"(?i){OPTIONAL_PRECEDING_WHITESPACE}({ANY_CHARACTER_REGEX})(!{}|{}!):",
            &*OPTIONAL_SCOPE_REGEX, &*OPTIONAL_SCOPE_REGEX
        )
        .as_str()
    )
    .unwrap();
    static ref MAJOR_FOOTER_INCREMENT_REGEX: Regex = Regex::new("\nBREAKING( |-)CHANGE:").unwrap();
    static ref PATCH_INCREMENT_REGEX: Regex = Regex::new(
        format!(
            r"(?i){OPTIONAL_PRECEDING_WHITESPACE}fix{}:",
            &*OPTIONAL_SCOPE_REGEX
        )
        .as_str()
    )
    .unwrap();
    static ref MINOR_INCREMENT_REGEX: Regex = Regex::new(
        format!(
            r"(?i){OPTIONAL_PRECEDING_WHITESPACE}feat{}:",
            &*OPTIONAL_SCOPE_REGEX
        )
        .as_str()
    )
    .unwrap();
}

pub(super) struct Commit {
    message: String,
}

impl Commit {
    pub(super) fn from_commit_message<T: Into<String>>(message: T) -> Commit {
        Commit {
            message: message.into(),
        }
    }

    pub(super) fn from_git(commit: &git2::Commit) -> Commit {
        let message = match commit.message().map(|m| m.to_string()) {
            Some(message) => {
                debug!(
                    "Found the commit message {message:?} for the commit with the hash '{}'.",
                    commit.id()
                );

                message
            }
            None => {
                warn!(
                    "Can not find commit message for the commit with the hash '{}'.",
                    commit.id()
                );

                String::new()
            }
        };

        Commit { message }
    }

    pub(super) fn is_major_increment(&self) -> bool {
        self.is_major_title_increment() || self.is_major_footer_increment()
    }

    fn is_major_title_increment(&self) -> bool {
        match MAJOR_TITLE_INCREMENT_REGEX.is_match(&self.message) {
            true => {
                debug!(
                    "{:?} matches a major title Semantic Versioning increment commit message.",
                    self.message
                );
                true
            }
            false => false,
        }
    }

    fn is_major_footer_increment(&self) -> bool {
        match MAJOR_FOOTER_INCREMENT_REGEX.is_match(&self.message) {
            true => {
                debug!(
                    "{:?} matches a major footer Semantic Versioning increment commit message.",
                    self.message
                );
                true
            }
            false => false,
        }
    }

    pub(super) fn is_minor_increment(&self) -> bool {
        match MINOR_INCREMENT_REGEX.is_match(&self.message) {
            true => {
                debug!(
                    "{:?} matches a minor Semantic Versioning increment commit message.",
                    self.message
                );
                true
            }
            false => false,
        }
    }

    pub(super) fn is_patch_increment(&self) -> bool {
        match PATCH_INCREMENT_REGEX.is_match(&self.message) {
            true => {
                debug!(
                    "{:?} matches a patch Semantic Versioning increment commit message.",
                    self.message
                );
                true
            }
            false => false,
        }
    }
}

#[cfg(test)]
mod tests;
