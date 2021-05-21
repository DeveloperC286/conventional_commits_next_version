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

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    let arguments = cli::Arguments::from_args();
    trace!("The command line arguments provided are {:?}.", arguments);

    let commit_messages = git::get_commit_messages_till_head_from(
        arguments.from_commit_hash,
        arguments.from_reference,
        arguments.monorepo,
    );

    let expected_version = increment::get_next_version_from_commits(
        commit_messages,
        arguments.from_version,
        arguments.batch_commits,
    );

    print!("{}", expected_version.to_string());

    if let Some(current_version) = arguments.current_version {
        if current_version < expected_version {
            std::process::exit(ERROR_EXIT_CODE);
        }
    }
}
