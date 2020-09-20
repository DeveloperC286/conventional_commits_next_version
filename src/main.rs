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
    let args = cli::Arguments::from_args();
    debug!("The command line arguemetns provided are {:?}.", args);

    let commit_messages = git::get_commit_messages_from(&args.from_commit_hash);
    let expected_version = increment::get_next_version_from_commits(
        commit_messages,
        args.from_version,
        args.batch_commits,
    );

    match args.current_version {
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
