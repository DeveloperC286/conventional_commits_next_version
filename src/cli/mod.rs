use semver::{Error, Version};
use structopt::{clap::ArgGroup, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "conventional_commits_next_version",
    about = "A tooling and language agnostic utility to calculate the next semantic version based on the Conventional Commits since the prior version. Supports monorepos.",
    group = ArgGroup::with_name("from").required(true)
)]
pub(crate) struct Arguments {
    #[structopt(
        long,
        group = "from",
        help = "The singular Git commit message to use in the calculation of the next semantic version."
    )]
    pub(crate) from_stdin: bool,

    #[structopt(
        long,
        group = "from",
        help = "The Git commit hash from where to start taking the range of commits from till HEAD. The range is inclusive of HEAD and exclusive of the provided Git commit hash."
    )]
    pub(crate) from_commit_hash: Option<String>,

    #[structopt(
        long,
        group = "from",
        help = "The Git reference from where to start taking the range of commits from till HEAD. The range is inclusive of HEAD and exclusive of the provided reference."
    )]
    pub(crate) from_reference: Option<String>,

    #[structopt(
        long,
        parse(try_from_str = parse_version),
        help = "The initial Semantic Versioning to begin calculations from."
    )]
    pub(crate) from_version: Version,

    #[structopt(
        long,
        help = "In batch mode the largest Semantic Versioning increment determined by the Conventional Commits type across all the commits is the only increment applied."
    )]
    pub(crate) batch_commits: bool,

    #[structopt(
        long,
        parse(try_from_str = parse_version),
        help = "This Semantic Versioning is asserted to be equal or larger than the calculated Semantic Versioning. The calculated Semantic Versioning is not printed to standard out. If the assertion is not met then it exits with a non zero exit code."
    )]
    pub(crate) current_version: Option<Version>,

    #[structopt(
        long,
        help = "The the next semantic version is calculated only from commits altering files which match any of these provided regexes, enabling usage within monorepos."
    )]
    pub(crate) monorepo: Vec<String>,
}

fn parse_version(src: &str) -> Result<Version, Error> {
    if src.starts_with('v') || src.starts_with('V') {
        return Version::parse(&src[1..]);
    }

    Version::parse(src)
}
