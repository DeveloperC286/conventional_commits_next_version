# Conventional Commits Next Version
[![crates.io](https://img.shields.io/crates/v/conventional_commits_next_version)](https://crates.io/crates/conventional_commits_next_version) [![crates.io_downloads](https://img.shields.io/crates/dv/conventional_commits_next_version)](https://crates.io/crates/conventional_commits_next_version) [![pipeline status](https://gitlab.com/DeveloperC/conventional_commits_next_version/badges/master/pipeline.svg)](https://gitlab.com/DeveloperC/conventional_commits_next_version/commits/master) [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

conventional_commits_next_version is a utility to calculate the next Semantic Versioning 2.0.0 (https://semver.org/spec/v2.0.0.html) based upon the supplied version and the Git commit history in the Conventional Commits v1.0.0 format (https://www.conventionalcommits.org/en/v1.0.0/).

## Why?
conventional_commits_next_version was created due to a lack of tooling not specific to and integrated with Node/NPM.
Following the Unix philosophy of 'Make each program do one thing well.' combined with utilising command line arguments and standard out; conventional_commits_next_version is a versatile tool not tied to specific tooling or language.

## Downloading Binary
For easy of use pre-compiled binaries are avaiable for download, so it can be easily used within CICD pipelines etc.


Each tag has its own binary built.
The downloaded file is zipped, unzip it and it produces a './conventional_commits_next_version' binary.
Update `<TAG>` to the desired tag you wish to download.

#### Note - You can find all the tags at `https://gitlab.com/DeveloperC/conventional_commits_next_version/-/tags`.

```
wget https://gitlab.com/DeveloperC/conventional_commits_next_version/-/jobs/artifacts/<TAG>/download?job=building-release
unzip download\?job=building-release
```

## Running
conventional_commits_next_version reads all the commit messages from HEAD in the current directory until the commit specified by the argument '--from-commit'.


### Consecutive

### Batch

## CICD Examples
### .gitlab-ci.yml stage for a Rust project.
```
merge-request-conventional-commits-next-version:
    stage: merge-request-conventional-commits-next-version
    image: rust
    script:
        # Get latest tag commit hash.
        - CURRENT_VERSION=`grep '^version = "[0-9].[0-9].[0-9]"$' Cargo.toml | cut -d '"' -f 2`
        - LATEST_TAG=`git tag -l | sort -r | head -1`
        - LATEST_TAG_HASH=`git rev-parse $LATEST_TAG`
        # Download conventional_commits_next_version.
        - cargo install conventional_commits_next_version
        # Compare current version vs expected.
        - $CARGO_HOME/bin/conventional_commits_next_version --batch-commits --from-commit-hash $LATEST_TAG_HASH --from-version $LATEST_TAG --current-version $CURRENT_VERSION
    rules:
        - if: $CI_MERGE_REQUEST_ID
```

## Compiling via Local Repository
Checkout the code repositroy locally, change into the repository's directory and then build via cargo.
Using the `--release` flag produces an optimised binary but takes longer to compile.

```
git clone git@gitlab.com:DeveloperC/conventional_commits_next_version.git
cd conventional_commits_next_version/
cargo build --release
```

The compiled binary is present in `target/release/conventional_commits_next_version`.

## Compiling via Cargo
Cargo is the Rust package manager, using the `install` sub-command it pulls the crate from `crates.io` and then compiles the binary locally.
Cargo install places the produced binary in `$HOME/.cargo/bin/conventional_commits_next_version`.

```
cargo install conventional_commits_next_version
```

## Unit Testing
The unit test suite has a number parametized tests testing the Conventional Commits v1.0.0 format parsing, cargo can be used to setup and run all the unit tests.

```
cargo test
```

## End-to-End Testing
To ensure correctness as there are a variety of out of process dependencies the project has an End-to-End test suite.
The End-to-End suite uses the behave framework (https://github.com/behave/behave).
To run the test suite you need to first build a binary, install behave and then execute behave.

#### Note - You can't use --release as the End-to-End test suite uses `target/debug/conventional_commits_next_version`.

```
cargo build
cd end-to-end-tests/
virtualenv -p python3 .venv
pip install -r requirements.txt
behave
```
