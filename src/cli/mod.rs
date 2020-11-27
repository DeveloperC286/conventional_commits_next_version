use semver::Version;
use structopt::{clap::ArgGroup, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "conventional_commits_next_version",
    about = "A tooling/language agnostic utility to calculate the next Semantic Versioning based upon the Conventional Commits Git commit messages since the last version.",
    group = ArgGroup::with_name("from").required(true)
)]
pub struct Arguments {
    #[structopt(
        long,
        group = "from",
        help = "The Git commit hash from where to start taking the range of commits from till HEAD. The range is inclusive of HEAD and exclusive of the provided Git commit hash."
    )]
    pub from_commit_hash: Option<git2::Oid>,
    #[structopt(
        long,
        group = "from",
        help = "The Git tag from where to start taking the range of commits from till HEAD. The range is inclusive of HEAD and exclusive of the provided tag."
    )]
    pub from_tag: Option<String>,
    #[structopt(
        long,
        help = "The initial Semantic Versioning to begin calculations from."
    )]
    pub from_version: Version,
    #[structopt(
        long,
        help = "If the flag is set only the single largest increment determined by the Git commit history of the Semantic Versioning is applied i.e. with one feature commit and one fix commit only the minor Semantic Versioning is increased."
    )]
    pub batch_commits: bool,
    #[structopt(
        long,
        help = "The `--current-version` Semantic Versioning is asserted to be equal or larger than the calculated Semantic Versioning. The calculated Semantic Versioning is not printed to standard out, if the assertion is not met then it exits with a non zero exit code."
    )]
    pub current_version: Option<Version>,
}
