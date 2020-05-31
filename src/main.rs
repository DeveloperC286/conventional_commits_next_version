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
    name = "conventional-commits-next-version",
    about = "A Rust utility to get the next semantic version based upon the current version supplied and the Git history in the conventional commit format."
)]
struct Args {
    #[structopt(
        long = "from-commit-hash",
        help = "The commit hash from where to use the Git commit messages to calculate the next version."
    )]
    from_commit_hash: String,
    #[structopt(
        long = "from-version",
        help = "The semantic versioning at the commit hash provided by the --from-commit-hash arguement."
    )]
    from_version: Version,
    #[structopt(
        long = "batch-commits",
        help = "The commits between HEAD and from-commit are batched together, only incrementing the semantic version by the biggest increase."
    )]
    batch_commits: bool,
    #[structopt(
        long = "current-version",
        help = "Instead of printing the expected semantic versioning, the current semantic versioning provided is compared to ensure it is equal or larger. If it does not meet this assertion the utility exits with a non zero returncode."
    )]
    current_version: Option<Version>,
}

fn main() {
    pretty_env_logger::init();
    let args = Args::from_args();

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
            if !(current_version >= expected_version) {
                std::process::exit(1);
            }
        }
        None => {
            print!("{}", expected_version.to_string());
        }
    }
}
