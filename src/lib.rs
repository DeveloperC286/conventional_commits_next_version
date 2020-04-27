use semver::Version;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod git;
mod increment;

pub fn get_next_version(from_commit: &str, version: &str) -> String {
    pretty_env_logger::init();
    let semantic_version = parse_semantic_version(version);
    let commit_messages = git::get_commit_messages_from(from_commit);

    return increment::get_next_version_from_commits(commit_messages, semantic_version).to_string();
}

fn parse_semantic_version(version: &str) -> Version {
    match Version::parse(version) {
        Ok(valid_semantic_version) => return valid_semantic_version,
        Err(_error) => {
            error!("'{}' is not a valid semantic version.", version);
            std::process::exit(1);
        }
    }
}
