#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::io::{stdin, Read};

use anyhow::{bail, Context, Result};
use clap::Parser;
use git2::Repository;

use crate::cli::Arguments;
use crate::commits::Commits;

mod calculation_mode;
mod cli;
mod commits;
mod history_mode;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    pretty_env_logger::init();
    trace!("Version {}.", env!("CARGO_PKG_VERSION"));
    let arguments = Arguments::parse();
    trace!("The command line arguments provided are {arguments:?}.");

    if let Err(err) = run(arguments) {
        error!("{:?}", err);
        std::process::exit(ERROR_EXIT_CODE);
    }
}

fn run(arguments: Arguments) -> Result<()> {
    let commits = if arguments.from == "-" {
        let mut commit_message = String::new();
        stdin().read_to_string(&mut commit_message).unwrap();

        Ok(Commits::from_commit_message(commit_message))
    } else {
        let repository =
            Repository::open_from_env().context("Unable to open the Git repository.")?;
        Commits::from_git(
            &repository,
            arguments.from,
            arguments.monorepo,
            arguments.history_mode,
        )
    }?;

    let expected_version =
        commits.get_next_version(arguments.from_version, arguments.calculation_mode);

    if let Some(current_version) = arguments.current_version {
        if current_version < expected_version {
            bail!(format!("The current version {current_version} is not larger or equal to the expected version {expected_version}."));
        }
        info!("The current version {current_version} is larger or equal to the expected version {expected_version}.");
    } else {
        print!("{expected_version}");
    }

    Ok(())
}
