use conventional_commits_next_version_lib::get_next_version;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "conventional-commits-next-version",
    about = "A Rust utility to get the next semantic version based upon the current version supplied and the Git history in the conventional commit format."
)]
struct Args {
    #[structopt(
        long = "from-commit",
        help = "The commit hash from where to use the Git commit messages to calculate the next version."
    )]
    from_commit: String,
    #[structopt(
        long = "version",
        help = "The current semantic version at the from commit hash provided."
    )]
    version: String,
    #[structopt(
        long = "batch-commits",
        help = "The commits between HEAD and from-commit are batched together, only incrementing the semantic version by the biggest increase."
    )]
    batch_commits: bool,
}

fn main() {
    let args = Args::from_args();
    print!(
        "{}",
        get_next_version(&args.from_commit, &args.version, args.batch_commits)
    );
}
