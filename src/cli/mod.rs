use semver::{SemVerError, Version};
use structopt::{clap::ArgGroup, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "conventional_commits_next_version",
    about = "A tooling and language agnostic utility to calculate the next semantic version based on the Conventional Commits since the prior version. Supports monorepos.",
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
        help = "The Git reference from where to start taking the range of commits from till HEAD. The range is inclusive of HEAD and exclusive of the provided reference."
    )]
    pub from_reference: Option<String>,

    #[structopt(
        long,
        parse(try_from_str = parse_version),
        help = "The initial Semantic Versioning to begin calculations from."
    )]
    pub from_version: Version,

    #[structopt(
        long,
        help = "In batch mode the single largest increment across all the Git commits in the Conventional Commits specification increments the Semantic Versioning."
    )]
    pub batch_commits: bool,

    #[structopt(
        long,
        parse(try_from_str = parse_version),
        help = "The `--current-version` Semantic Versioning is asserted to be equal or larger than the calculated Semantic Versioning. The calculated Semantic Versioning is not printed to standard out, if the assertion is not met then it exits with a non zero exit code."
    )]
    pub current_version: Option<Version>,

    #[structopt(
        long,
        help = "Monorepo usage is supported, the version is calculated only from commits altering the specified directory."
    )]
    pub monorepo: Option<String>,
}

fn parse_version(src: &str) -> Result<Version, SemVerError> {
    if src.starts_with('v') || src.starts_with('V') {
        return Version::parse(&src[1..]);
    }

    Version::parse(src)
}
