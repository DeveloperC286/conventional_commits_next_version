use git2::{Oid, Repository};
use regex::Regex;
use std::process::exit;

use crate::model::monorepo::Monorepo;

const OPTIONAL_PRECEDING_WHITESPACE: &str = "^([[:space:]])*";
const ANY_CHARACTER_REGEX: &str = "([[:digit:]]|[[:alpha:]])+";
const OPTIONAL_ANY_REGEX: &str = "([[:digit:]]|[[:alpha:]]|_|-|[[:space:]])*";

lazy_static! {
    static ref OPTIONAL_SCOPE_REGEX: String = format!(r"(\({}\))?", OPTIONAL_ANY_REGEX);
    static ref MAJOR_TITLE_INCREMENT_REGEX: Regex = Regex::new(
        format!(
            r"(?i){}({})(!{}|{}!):",
            &*OPTIONAL_PRECEDING_WHITESPACE,
            &*ANY_CHARACTER_REGEX,
            &*OPTIONAL_SCOPE_REGEX,
            &*OPTIONAL_SCOPE_REGEX
        )
        .as_str()
    )
    .unwrap();
    static ref MAJOR_FOOTER_INCREMENT_REGEX: Regex = Regex::new("\nBREAKING( |-)CHANGE:").unwrap();
    static ref PATCH_INCREMENT_REGEX: Regex = Regex::new(
        format!(
            r"(?i){}fix{}:",
            &*OPTIONAL_PRECEDING_WHITESPACE, &*OPTIONAL_SCOPE_REGEX
        )
        .as_str()
    )
    .unwrap();
    static ref MINOR_INCREMENT_REGEX: Regex = Regex::new(
        format!(
            r"(?i){}feat{}:",
            &*OPTIONAL_PRECEDING_WHITESPACE, &*OPTIONAL_SCOPE_REGEX
        )
        .as_str()
    )
    .unwrap();
}

pub struct Commit {
    message: String,
}

impl Commit {
    pub fn from(repository: &Repository, oid: Oid, monorepo: &Monorepo) -> Option<Self> {
        match repository.find_commit(oid) {
            Ok(commit) => match commit.message().map(|m| m.to_string()) {
                Some(message) => {
                    trace!(
                        "Found the commit message {:?} for the commit with the hash '{}'.",
                        message,
                        commit.id()
                    );

                    match monorepo.does_commit_effect(repository, commit) {
                        true => Some(Commit { message }),
                        false => None,
                    }
                }
                None => {
                    error!(
                        "Can not find commit message for the commit with the hash '{}'.",
                        oid
                    );
                    None
                }
            },
            Err(error) => {
                error!("{:?}", error);
                error!("Can not find commit with the hash '{}'.", oid);
                exit(crate::ERROR_EXIT_CODE);
            }
        }
    }

    pub fn is_major_increment(&self) -> bool {
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

    pub fn is_minor_increment(&self) -> bool {
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

    pub fn is_patch_increment(&self) -> bool {
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
