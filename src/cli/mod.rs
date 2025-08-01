use clap::Parser;
use semver::{Error, Version};

pub use crate::calculation_mode::CalculationMode;
pub use crate::history_mode::HistoryMode;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Arguments {
    #[arg(
        long,
        help = "The the next semantic version is calculated only from commits altering files which match any of these provided regexes, enabling usage within monorepos."
    )]
    pub(crate) monorepo: Vec<String>,

    #[arg(
        long,
        default_value = "first",
        help = "Specifies how commits are parsed, acceptable values are 'first' to parse only the first parent of merge commits, or 'all' to parse all parents."
    )]
    pub(crate) history_mode: HistoryMode,

    #[arg(
        long,
        value_parser = parse_version,
        help = "The initial semantic version to begin the calculations from."
    )]
    pub(crate) from_version: Version,

    #[arg(
        long,
        default_value = "consecutive",
        help = "The mode of calculation to use on the range of Commits to calculate the next semantic version."
    )]
    pub(crate) calculation_mode: CalculationMode,

    #[arg(
        long,
        value_parser = parse_version,
        help = "This semantic version is asserted to be equal or larger than the calculated semantic version. The calculated semantic version is not printed to standard out. If the assertion is not met then it exits with a non zero exit code."
    )]
    pub(crate) current_version: Option<Version>,

    #[arg(
        long,
        help = "Enable verbose output, respects RUST_LOG environment variable if set."
    )]
    pub(crate) verbose: bool,

    #[arg(
        help = "The Git reference from where to start taking the range of commits from till HEAD to lint. The range is inclusive of HEAD and exclusive of the provided reference. '-' indicates to read the standard input and lint the input as a Git commit message."
    )]
    pub(crate) from: String,
}

fn parse_version(src: &str) -> Result<Version, Error> {
    if src.starts_with('v') || src.starts_with('V') {
        return Version::parse(&src[1..]);
    }

    Version::parse(src)
}
