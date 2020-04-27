use conventional_commits_next_version_lib::get_next_version;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "conventional-commits-next-version",
    about = "A Rust utility to get the next semantic version based upon the current version provided and the Git history to read from."
)]
struct Args {
    #[structopt(
        long = "from-commit",
        help = "The commit hash the current version comes from and from where to use the Git commits to calculate the next version."
    )]
    from_commit: String,
    #[structopt(
        long = "version",
        help = "The semantic version at the from commit hash provided."
    )]
    version: String,
}

fn main() {
    let args = Args::from_args();
    print!("{}", get_next_version(&args.from_commit, &args.version));
}
