use regex::Regex;

const OPTIONAL_PRECEDING_WHITESPACE: &str = "^([[:space:]])*";
const ANY_REGEX: &str = "([[:digit:]]|[[:alpha:]]|_|-|[[:space:]])*";

lazy_static! {
    static ref OPTIONAL_SCOPE_REGEX: String = format!(r"(\({}\))?", ANY_REGEX);
    pub(super) static ref MAJOR_TITLE_INCREMENT_REGEX: Regex = Regex::new(
        format!(
            r"(?i){}({})(!{}|{}!):",
            &*OPTIONAL_PRECEDING_WHITESPACE,
            &*ANY_REGEX,
            &*OPTIONAL_SCOPE_REGEX,
            &*OPTIONAL_SCOPE_REGEX
        )
        .as_str()
    )
    .unwrap();
    pub(super) static ref MAJOR_FOOTER_INCREMENT_REGEX: Regex = Regex::new(
        format!(
            r"(?i)^({}){}:(.)*(\n)*BREAKING CHANGE:",
            &*ANY_REGEX, &*OPTIONAL_SCOPE_REGEX
        )
        .as_str()
    )
    .unwrap();
    pub(super) static ref PATCH_INCREMENT_REGEX: Regex = Regex::new(
        format!(
            r"(?i){}fix{}:",
            &*OPTIONAL_PRECEDING_WHITESPACE, &*OPTIONAL_SCOPE_REGEX
        )
        .as_str()
    )
    .unwrap();
    pub(super) static ref MINOR_INCREMENT_REGEX: Regex = Regex::new(
        format!(
            r"(?i){}feat{}:",
            &*OPTIONAL_PRECEDING_WHITESPACE, &*OPTIONAL_SCOPE_REGEX
        )
        .as_str()
    )
    .unwrap();
}
