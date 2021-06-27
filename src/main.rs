#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate regex;

use structopt::StructOpt;

use crate::model::commits::Commits;

mod cli;
mod model;
mod utilities;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    trace!("Version {}.", env!("CARGO_PKG_VERSION"));
    let arguments = cli::Arguments::from_args();
    trace!("The command line arguments provided are {:?}.", arguments);

    let commits = Commits::from(
        arguments.from_commit_message,
        arguments.from_commit_hash,
        arguments.from_reference,
        arguments.monorepo,
    );

    let expected_version =
        commits.get_next_version(arguments.from_version, arguments.batch_commits);

    if let Some(current_version) = arguments.current_version {
        if current_version < expected_version {
            error!(
                "The current version {} is not larger or equal to the expected version {}.",
                current_version, expected_version
            );
            std::process::exit(ERROR_EXIT_CODE);
        }
        info!(
            "The current version {} is larger or equal to the expected version {}.",
            current_version, expected_version
        );
    } else {
        print!("{}", expected_version.to_string());
    }
}
