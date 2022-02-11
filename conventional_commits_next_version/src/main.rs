#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::io::{stdin, Read};
use std::process::exit;

use conventional_commits_next_version_lib::Commits;
use git2::Repository;
use structopt::StructOpt;

mod cli;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    trace!("Version {}.", env!("CARGO_PKG_VERSION"));
    let arguments = cli::Arguments::from_args();
    trace!("The command line arguments provided are {arguments:?}.");

    match Repository::open_from_env() {
        Ok(repository) => {
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
            };

            match commits {
                Ok(commits) => {
                    let expected_version =
                        commits.get_next_version(arguments.from_version, arguments.batch_commits);

                    if let Some(current_version) = arguments.current_version {
                        if current_version < expected_version {
                            error!(
                "The current version {current_version} is not larger or equal to the expected version {expected_version}."
            );
                            exit(ERROR_EXIT_CODE);
                        }
                        info!(
                            "The current version {current_version} is larger or equal to the expected version {expected_version}."
                        );
                    } else {
                        print!("{expected_version}");
                    }
                }
                Err(_) => {
                    exit(ERROR_EXIT_CODE);
                }
            }
        }
        Err(_) => {
            error!("Failed to open a Git repository from the current directory or Git environment variables.");
            exit(ERROR_EXIT_CODE);
        }
    }
}
