# Conventional Commits Next Version
[![crates.io](https://img.shields.io/crates/v/conventional_commits_next_version)](https://crates.io/crates/conventional_commits_next_version) [![pipeline status](https://gitlab.com/DeveloperC/conventional_commits_next_version/badges/master/pipeline.svg)](https://gitlab.com/DeveloperC/conventional_commits_next_version/commits/master) [![License: AGPL v3](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)


A tooling/language agnostic utility to calculate the next Semantic Versioning based upon the Conventional Commits Git commit messages since the last version.


## Why use Conventional Commits Next Version?
 * __Tooling/Language Agnostic__ - Only command line arguments are utilised for input, no inbuilt functionality parses tooling/language specific files to obtain information.
 * __No Dependencies__ - A binary download is provided, removing dependencies on downloading tools or interpreter languages.
 * __Not A Linter__ - Minor deviations from the Conventional Commits format are still accepted as valid input because this is not a linter (but we suggest you use [Conventional Commits Linter](https://gitlab.com/DeveloperC/conventional_commits_linter)).
 * __Invalid Commits Are Ignored__ - Only warnings are logged when Non Conventional Commits are encountered, the calculation continues performing.


## Content
 * [Usage](#usage)
   + [Usage - Consecutive Mode](#usage-consecutive-mode)
   + [Usage - Batch Mode](#usage-batch-mode)
   + [Usage - Logging](#usage-logging)
 * [CICD Examples](#cicd-examples)
   + [GitLab CI Rust Project Example](#gitlab-ci-rust-project-example)
     + [Via Cargo](#via-cargo)
     + [Via Binary Download](#via-binary-download)
 * [Downloading Binary](#downloading-binary)
 * [Compiling via Local Repository](#compiling-via-local-repository)
 * [Compiling via Cargo](#compiling-via-cargo)
 * [Unit Testing](#unit-testing)
 * [End-to-End Testing](#end-to-end-testing)
 * [Issues/Feature Requests](#issuesfeature-requests)


## Usage
Through the non-optional arguments `--from-commit-hash` and `--from-version` the commit messages are parsed against the Conventional Commits v1.0.0 specification.
The Conventional Commits types of the commit messages are used to increment the Semantic Versioning provided via `--from-version` and is printed to standard out.

conventional_commits_next_version finds and open an existing repository, respecting git environment variables.
With $GIT_DIR unset, this will search for a repository starting in the current directory.

The optional `--current-version` Semantic Versioning argument can be provided.
The Semantic Versioning provided is asserted to be equal or larger than the calculated next Semantic Versioning.
The calculated next Semantic Versioning is not printed to standard out and if the assertion is not meet then it exits with a non zero exit code.

Two different modes can be used by conventional_commits_next_version when calculating the next Semantic Versioning, both are described below.


### Usage - Consecutive Mode
By default conventional_commits_next_version operates in a consecutive manner.
Each commit Conventional Commits type in the order they were committed started at the commit hash at `--from-commit-hash` is used to increment the Semantic Versioning provided via `--from-version`.

e.g.

```
git clone https://github.com/yargs/yargs.git
cd yargs
git checkout 3af7f04cdbfcbd4b3f432aca5144d43f21958c39
RUST_LOG=trace conventional_commits_next_version --from-commit-hash a5edc328ecb3f90d1ba09cfe70a0040f68adf50a --from-version 1.13.2
```

Using the environment variable `RUST_LOG` we can enable more detailed logging, so we can see the internal logic.

```
DEBUG conventional_commits_next_version::increment > Incrementing semantic versioning patch because of commit "fix: address ambiguity between nargs of 1 and requiresArg (#1572)\n\n".
```

From the logs we can see that the commit `a5edc328ecb3f90d1ba09cfe70a0040f68adf50a` has the Conventional Commits type of `fix`.
The fix type will cause the increment of the initial Semantic Versioning provided via `--from-version` from `1.13.2` to `1.13.3`.

```
DEBUG conventional_commits_next_version::increment > Incrementing semantic versioning minor because of commit "feat(yargs-parser): introduce single-digit boolean aliases (#1576)\n\n".
```

From the logs we can see that the commit `3af7f04cdbfcbd4b3f432aca5144d43f21958c39` has the Conventional Commits type of `feat`.
The feat type encountered will increment the minor Semantic Versioning from `1.13.3` to `1.14.0`.

```
DEBUG conventional_commits_next_version::increment > Incrementing semantic versioning minor because of commit "feat: add usage for single-digit boolean aliases (#1580)\n\n".
```

From the logs we can see that the commit `6014e39bca3a1e8445aa0fb2a435f6181e344c451` has the Conventional Commits type of `feat`.
The feat type will increment the minor Semantic Versioning from `1.14.0` to `1.15.0`.
All the other commits will be parsed and the next Semantic Versioning  will be printed the standard out and can then be used as input for other tools.

```
> 1.15.0
```


### Usage - Batch Mode
conventional_commits_next_version can be told to batching together the commits with the addition of the `--batch-commits` flag on the command line.
This causes only the single largest Semantic Versioning to be applied.
i.e. with one feature commit and one fix commit only the minor Semantic Versioning is increased.
This is useful for when a merge is not being compressed into a singular commit, but the branch's being merged commits are being rebased onto master.

e.g.

```
git clone https://github.com/yargs/yargs.git
cd yargs
git checkout 3af7f04cdbfcbd4b3f432aca5144d43f21958c39
RUST_LOG=trace conventional_commits_next_version --from-commit-hash a5edc328ecb3f90d1ba09cfe70a0040f68adf50a --from-version 1.13.2 --batch-commits
```

Using the same example but with the `--batch-commits` flag appended, we can see how batch behaves differently.

```
DEBUG conventional_commits_next_version::increment > Incrementing semantic versioning minor because of commit "feat(yargs-parser): introduce single-digit boolean aliases (#1576)\n\n".
DEBUG conventional_commits_next_version::increment > Incrementing semantic versioning minor because of commit "feat: add usage for single-digit boolean aliases (#1580)\n\n".
```

We can that the two largest Conventional Commits types found were `feat`.
The feat type encountered will increment the minor Semantic Versioning from `1.13  to `1.14.0`.
The order does not matter, nor do any of the other commits types.
You can see the commit with the type `fix` has been ignored.
The next Semantic Versioning  will be printed the standard out and can then be used as input for other tools.

```
> 1.14.0
```


### Usage - Logging
The crates `pretty_env_logger` and `log` are used to provide logging.
The environment variable `RUST_LOG` can be used to set the logging level.
See [https://crates.io/crates/pretty_env_logger](https://crates.io/crates/pretty_env_logger) for more detailed documentation.


## CICD Examples
### GitLab CI Rust Project Example
#### Via Cargo
```
conventional-commits-next-version-checking:
    stage: conventional-commits-next-version-checking
    image: rust
    before_script:
        - cargo install conventional_commits_next_version
    script:
        # Get current version and latest tag.
        - CURRENT_VERSION=`grep '^version = "[0-9][0-9]*.[0-9][0-9]*.[0-9][0-9]*"$' Cargo.toml | cut -d '"' -f 2`
        # Get latest tag.
        - LATEST_TAG=`git describe --tags | cut -d '-' -f 1`
        - LATEST_TAG_HASH=`git rev-parse $LATEST_TAG`
        # Check latest tag is in semantic versioning.
        - echo $LATEST_TAG | grep "^[0-9][0-9]*.[0-9][0-9]*.[0-9][0-9]*$"
        # Check current vs expected.
        - /usr/local/cargo/bin/conventional_commits_next_version --batch-commits --from-commit-hash $LATEST_TAG_HASH --from-version $LATEST_TAG --current-version $CURRENT_VERSION
    rules:
        - if: $CI_MERGE_REQUEST_ID
```


#### Via Binary Download
The two differences are minor, the first is the step installing from Cargo is replaced with downloading and unzipping the binary and the second is changing the path the binary is called from.

```
conventional-commits-next-version-checking:
    stage: conventional-commits-next-version-checking
    image: rust
    before_script:
        - wget -q -O tmp.zip "https://gitlab.com/DeveloperC/conventional_commits_next_version/-/jobs/artifacts/1.1.0/download?job=building-release-binary-linux-musl" && unzip tmp.zip && rm tmp.zip
    script:
        # Get current version and latest tag.
        - CURRENT_VERSION=`grep '^version = "[0-9][0-9]*.[0-9][0-9]*.[0-9][0-9]*"$' Cargo.toml | cut -d '"' -f 2`
        # Get latest tag.
        - LATEST_TAG=`git describe --tags | cut -d '-' -f 1`
        - LATEST_TAG_HASH=`git rev-parse $LATEST_TAG`
        # Check latest tag is in semantic versioning.
        - echo $LATEST_TAG | grep "^[0-9][0-9]*.[0-9][0-9]*.[0-9][0-9]*$"
        # Check current vs expected.
        - ./conventional_commits_next_version --batch-commits --from-commit-hash $LATEST_TAG_HASH --from-version $LATEST_TAG --current-version $CURRENT_VERSION
    rules:
        - if: $CI_MERGE_REQUEST_ID
```


## Downloading Binary
Statically linked compiled binaries are available for download.
Visit the releases page at [https://gitlab.com/DeveloperC/conventional_commits_next_version/-/releases](https://gitlab.com/DeveloperC/conventional_commits_next_version/-/releases) to see all the releases, the release notes contains links to binary downloads for various architectures.


## Compiling via Local Repository
Checkout the code repository locally, change into the repository's directory and then build via cargo.
Using the `--release` flag produces an optimised binary but takes longer to compile.

```
git clone git@gitlab.com:DeveloperC/conventional_commits_next_version.git
cd conventional_commits_next_version/
cargo build --release
```

The compiled binary is present in `target/release/conventional_commits_next_version`.


## Compiling via Cargo
Cargo is the Rust package manager, using the `install` sub-command it pulls the crate from `crates.io` and then compiles the binary locally.
`cargo install` places the produced binary at `$HOME/.cargo/bin/conventional_commits_next_version`.

```
cargo install conventional_commits_next_version
```


## Unit Testing
The unit test suite has a number parameterised tests testing the Conventional Commits v1.0.0 format parsing, cargo can be used to setup and run all the unit tests.

```
cargo test
```


## End-to-End Testing
To ensure correctness as there are a variety of out of process dependencies the project has an End-to-End behaviour driven test suite using the behave framework (https://github.com/behave/behave).
To run the test suite you need to first build a binary, install Python3, install behave and then execute behave to run the behaviour driven test suite.



#### Note - You can't use --release as the End-to-End test suite uses `target/debug/conventional_commits_next_version`.

```
cargo build
cd end-to-end-tests/
virtualenv -p python3 .venv
source .venv/bin/activate
pip3 install -r requirements.txt
behave
```


## Issues/Feature Requests
To report an issue or request a new feature use [https://gitlab.com/DeveloperC/conventional_commits_next_version/-/issues](https://gitlab.com/DeveloperC/conventional_commits_next_version/-/issues).
