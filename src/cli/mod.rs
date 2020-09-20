use semver::Version;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "conventional_commits_next_version",
    about = "A tooling/language agnostic utility to calculate the next Semantic Versioning based upon the Conventional Commits Git commit messages since the last version."
)]
pub struct Arguments {
    #[structopt(
        long = "from-commit-hash",
        help = "The Git commit hash from where to start using the commit messages till HEAD."
    )]
    pub from_commit_hash: String,
    #[structopt(
        long = "from-version",
        help = "The Semantic Versioning at the Git commit hash provided by the --from-commit-hash argument."
    )]
    pub from_version: Version,
    #[structopt(
        long = "batch-commits",
        help = "If the flag is set only the single largest increment determined by the Git commit history of the Semantic Versioning is applied i.e. with one feature commit and one fix commit only the minor Semantic Versioning is increased."
    )]
    pub batch_commits: bool,
    #[structopt(
        long = "current-version",
        help = "The provided Semantic Versioning is asserted to be equal or larger than the calculated next Semantic Versioning. The calculated next Semantic Versioning is not printed to standard out and if the assertion is not meet then it exits with a non zero exit code."
    )]
    pub current_version: Option<Version>,
}
