use semver::Version;
use structopt::StructOpt;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod git;
mod increment;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "conventional_commits_next_version",
    about = "A tooling/language agnostic utility to calculate the next Semantic Versioning based upon the Conventional Commits Git commit messages since the last version."
)]
struct Args {
    #[structopt(
        long = "from-commit-hash",
        help = "The Git commit hash from where to start using the commit messages till HEAD."
    )]
    from_commit_hash: String,
    #[structopt(
        long = "from-version",
        help = "The Semantic Versioning at the Git commit hash provided by the --from-commit-hash argument."
    )]
    from_version: Version,
    #[structopt(
        long = "batch-commits",
        help = "If the flag is set only the single largest increment determined by the Git commit history of the Semantic Versioning is applied i.e. with one feature commit and one fix commit only the minor Semantic Versioning is increased."
    )]
    batch_commits: bool,
    #[structopt(
        long = "current-version",
        help = "The provided Semantic Versioning is asserted to be equal or larger than the calculated next Semantic Versioning. The calculated next Semantic Versioning is not printed to standard out and if the assertion is not meet then it exits with a non zero exit code."
    )]
    current_version: Option<Version>,
}

fn main() {
    pretty_env_logger::init();
    let args = Args::from_args();
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
