#[macro_use]
extern crate lazy_static;

use std::io::{stdin, Read};

use anyhow::{bail, Context, Result};
use clap::Parser;
use git2::Repository;
use log::{debug, error, info};

use crate::cli::Arguments;
use crate::commits::Commits;

mod calculation_mode;
mod cli;
mod commits;
mod history_mode;

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    let arguments = Arguments::parse();

    // Set up logging. Log level precedence:
    // - RUST_LOG, if set.
    // - info, if --verbose is passed.
    let mut logger = pretty_env_logger::formatted_builder();
    match std::env::var("RUST_LOG") {
        Ok(rust_log) => {
            logger.parse_filters(&rust_log);
        }
        Err(_) if arguments.verbose => {
            logger.filter_level(log::LevelFilter::Info);
        }
        Err(_) => {}
    }
    logger.init();

    info!("Version {}.", env!("CARGO_PKG_VERSION"));
    debug!("The command line arguments provided are {arguments:?}.");

    if let Err(err) = run(arguments) {
        error!("{err:?}");
        std::process::exit(ERROR_EXIT_CODE);
    }
}

fn run(arguments: Arguments) -> Result<()> {
    let commits = if arguments.from == "-" {
        let mut commit_message = String::new();
        stdin()
            .read_to_string(&mut commit_message)
            .context("Unable to read the commit message from standard input.")?;

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
