use clap::{ArgGroup, Parser};
use semver::{Error, Version};

pub use crate::calculation_mode::CalculationMode;
pub use crate::git_history_mode::GitHistoryMode;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("from")
        .required(true)
))]
pub(crate) struct Arguments {
    #[arg(
        long,
        group = "from",
        help = "The singular Git commit message to use in the calculation of the next semantic version."
    )]
    pub(crate) from_stdin: bool,

    #[arg(
        long,
        group = "from",
        help = "The Git reference from where to start taking the range of commits from till HEAD. The range is inclusive of HEAD and exclusive of the provided reference."
    )]
    pub(crate) from_reference: Option<String>,

    #[arg(
        long,
        group = "from",
        help = "The Git commit hash from where to start taking the range of commits from till HEAD. The range is inclusive of HEAD and exclusive of the provided Git commit hash."
    )]
    pub(crate) from_commit_hash: Option<String>,

    #[arg(
        long,
        help = "The the next semantic version is calculated only from commits altering files which match any of these provided regexes, enabling usage within monorepos."
    )]
    pub(crate) monorepo: Vec<String>,

    #[arg(
        long,
        default_value = "FirstParent",
        help = "The mode to use when transversing the Git commit history of the Git commit range, to collect the Git commit messages to use in calculating the next semantic version."
    )]
    pub(crate) git_history_mode: GitHistoryMode,

    #[arg(
        long,
        value_parser = parse_version,
        help = "The initial semantic version to begin the calculations from."
    )]
    pub(crate) from_version: Version,

    #[arg(
        long,
        default_value = "Consecutive",
        help = "The mode of calculation to use on the range of Commits to calculate the next semantic version."
    )]
    pub(crate) calculation_mode: CalculationMode,

    #[arg(
        long,
        value_parser = parse_version,
        help = "This semantic version is asserted to be equal or larger than the calculated semantic version. The calculated semantic version is not printed to standard out. If the assertion is not met then it exits with a non zero exit code."
    )]
    pub(crate) current_version: Option<Version>,
}

fn parse_version(src: &str) -> Result<Version, Error> {
    if src.starts_with('v') || src.starts_with('V') {
        return Version::parse(&src[1..]);
    }

    Version::parse(src)
}
