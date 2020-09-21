#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate regex;

use structopt::StructOpt;

mod cli;
mod git;
mod increment;

fn main() {
    pretty_env_logger::init();
    let arguments = cli::Arguments::from_args();
    debug!("The command line arguments provided are {:?}.", arguments);

    let commit_messages =
        git::get_commit_messages_till_head_from(arguments.from_commit_hash, arguments.from_tag);

    let expected_version = increment::get_next_version_from_commits(
        commit_messages,
        arguments.from_version,
        arguments.batch_commits,
    );

    match arguments.current_version {
        Some(current_version) => {
            debug!(
                "Comparing current_version {} >= expected_version {}.",
                current_version.to_string(),
                expected_version.to_string()
            );
            if current_version < expected_version {
                std::process::exit(1);
            }
        }
        None => {
            print!("{}", expected_version.to_string());
        }
    }
}
