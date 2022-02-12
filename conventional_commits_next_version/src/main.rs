#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::io::{stdin, Read};

use conventional_commits_next_version_lib::Commits;
use git2::Repository;
use structopt::StructOpt;

use crate::cli::Arguments;

mod cli;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    trace!("Version {}.", env!("CARGO_PKG_VERSION"));
    let arguments = Arguments::from_args();
    trace!("The command line arguments provided are {arguments:?}.");

    if let Err(_) = run(arguments) {
        std::process::exit(ERROR_EXIT_CODE);
    }
}

fn run(arguments: Arguments) -> Result<(), git2::Error> {
    let repository = Repository::open_from_env()?;
    let commits = match (
        arguments.from_stdin,
        arguments.from_commit_hash,
        arguments.from_reference,
    ) {
        (true, None, None) => {
            let mut commit_message = String::new();
            stdin().read_to_string(&mut commit_message).unwrap();

            Ok(Commits::from_commit_message(commit_message))
        }
        (false, Some(from_commit_hash), None) => {
            Commits::from_commit_hash(&repository, &from_commit_hash, arguments.monorepo)
        }
        (false, None, Some(from_reference)) => {
            Commits::from_reference(&repository, &from_reference, arguments.monorepo)
        }
        (_, _, _) => {
            unreachable!(
                "Invalid combination of from arguments, should have been caught by structopt."
            );
        }
    }?;
    let expected_version =
        commits.get_next_version(arguments.from_version, arguments.batch_commits);

    if let Some(current_version) = arguments.current_version {
        if current_version < expected_version {
            let error_message = format!("The current version {current_version} is not larger or equal to the expected version {expected_version}.");
            error!("{error_message}");
            return Err(git2::Error::from_str(&error_message));
        }
        info!("The current version {current_version} is larger or equal to the expected version {expected_version}.");
    } else {
        print!("{expected_version}");
    }

    Ok(())
}