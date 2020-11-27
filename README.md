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
   + [Usage - Git Environment Variables](#usage-git-environment-variables)
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
conventional_commits_next_version collects Git commits directly from a Git repository(See [Usage - Git Environment Variables](#usage-git-environment-variables)).
You specify from where exclusively to begin collecting using either `--from-commit-hash` or `--from-tag`.

Any Git commits meeting the Conventional Commits v1.0.0 specification are then used to calculate the next Semantic Versioning.
The initial Semantic Versioning to begin calculations from is provided via `--from-version`.

By default the next calculate version is printed to standard out.
However if you provide the optional `--current-version` Semantic Versioning argument.
The `--current-version` Semantic Versioning is asserted to be equal or larger than the calculated Semantic Versioning.
The calculated Semantic Versioning is not printed to standard out, if the assertion is not met then it exits with a non zero exit code.

There are two modes of calculating the next Semantic Versioning consecutive mode and batch mode.
By default Consecutive mode is used.


### Usage - Consecutive Mode
In consecutive mode each Git commit in the Conventional Commits specification is applied to Semantic Versioning calculation in chronological order.

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

There are no more Conventional Commits which will increment the Semantic Versioning, so the calculated Semantic Versioning is printed to standard out.

```
> 1.15.0
```


### Usage - Batch Mode
In batch mode the single largest increment across all the Git commits in the Conventional Commits specification increments the Semantic Versioning.

Batch mode is enabled via the `--batch-commits` flag.

Batch mode is useful for feature branches, if it has multiple fixes or a mixture of fixes and features being merged.

e.g.

```
git clone https://github.com/yargs/yargs.git
cd yargs
git checkout 3af7f04cdbfcbd4b3f432aca5144d43f21958c39
RUST_LOG=trace conventional_commits_next_version --from-commit-hash a5edc328ecb3f90d1ba09cfe70a0040f68adf50a --from-version 1.13.2 --batch-commits
```

Using the same example we can see how batch mode behaves differently.

```
DEBUG conventional_commits_next_version::increment > Incrementing semantic versioning minor because of commit "feat(yargs-parser): introduce single-digit boolean aliases (#1576)\n\n".
DEBUG conventional_commits_next_version::increment > Incrementing semantic versioning minor because of commit "feat: add usage for single-digit boolean aliases (#1580)\n\n".
```

The largest increment is a minor Semantic Versioning increment, because of the two Conventional Commits with `feat` as the type.
The minor Semantic Versioning increment increases the initial Semantic Versioning from `1.13.0` to `1.14.0`.

The calculated Semantic Versioning is then printed to standard out.

```
> 1.14.0
```


### Usage - Git Environment Variables
When looking for a repository the Git environment variables are respected.
When `$GIT_DIR` is set then it takes precedence and Conventional Commits Next Version begins searching for a repository in the directory specified in `$GIT_DIR`.
When `$GIT_DIR` is not set then Conventional Commits Next Version searches for a repository begins in the current directory.


### Usage - Logging
The crates `pretty_env_logger` and `log` are used to provide logging.
The environment variable `RUST_LOG` can be used to set the logging level.
See [https://crates.io/crates/pretty_env_logger](https://crates.io/crates/pretty_env_logger) for more detailed documentation.


## CICD Examples
### GitLab CI Rust Project Example
#### Via Cargo
See [Compiling via Cargo](#compiling-via-cargo) for more details about installing via Cargo.

__Note - This example downloads the latest version at the time of execution.__

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
        # Check latest tag is in semantic versioning.
        - echo $LATEST_TAG | grep "^[0-9][0-9]*.[0-9][0-9]*.[0-9][0-9]*$"
        # Check current vs expected.
        - /usr/local/cargo/bin/conventional_commits_next_version --batch-commits --from-tag $LATEST_TAG --from-version $LATEST_TAG --current-version $CURRENT_VERSION
    rules:
        - if: $CI_MERGE_REQUEST_ID
```


#### Via Binary Download
See [Downloading Binary](#downloading-binary) for more details about Binary downloads.

__Note - This example downloads version `1.4.0`.__

```
conventional-commits-next-version-checking:
    stage: conventional-commits-next-version-checking
    image: rust
    before_script:
        - wget -q -O tmp.zip "https://gitlab.com/DeveloperC/conventional_commits_next_version/-/jobs/artifacts/1.4.0/download?job=release-binary-compiling-x86_64-linux-musl" && unzip tmp.zip && rm tmp.zip
    script:
        # Get current version and latest tag.
        - CURRENT_VERSION=`grep '^version = "[0-9][0-9]*.[0-9][0-9]*.[0-9][0-9]*"$' Cargo.toml | cut -d '"' -f 2`
        # Get latest tag.
        - LATEST_TAG=`git describe --tags | cut -d '-' -f 1`
        # Check latest tag is in semantic versioning.
        - echo $LATEST_TAG | grep "^[0-9][0-9]*.[0-9][0-9]*.[0-9][0-9]*$"
        # Check current vs expected.
        - ./conventional_commits_next_version --batch-commits --from-tag $LATEST_TAG --from-version $LATEST_TAG --current-version $CURRENT_VERSION
    rules:
        - if: $CI_MERGE_REQUEST_ID
```


## Downloading Binary
Statically linked compiled binaries are available for download.
Visit the releases page at [https://gitlab.com/DeveloperC/conventional_commits_next_version/-/releases](https://gitlab.com/DeveloperC/conventional_commits_next_version/-/releases) to see all the releases, the release notes contains links to binary downloads for various architectures.

If you do not trust the provided binaries another option is to compile your own and then make it available for remote download, so your CICD etc can then download it.


## Compiling via Local Repository
Checkout the code repository locally, change into the repository's directory and then build via Cargo.
Using the `--release` flag produces an optimised binary but takes longer to compile.

```
git clone git@gitlab.com:DeveloperC/conventional_commits_next_version.git
cd conventional_commits_next_version/
cargo build --release
```

The compiled binary is present in `target/release/conventional_commits_next_version`.


## Compiling via Cargo
Cargo is the Rust package manager, using the `install` sub-command it pulls the Conventional Commits Next Version from `crates.io` and then compiles the binary locally.
`cargo install` places the produced binary at `$HOME/.cargo/bin/conventional_commits_next_version`.

```
cargo install conventional_commits_next_version
```

You can specify a specific version to install using the `--version` argument.
Otherwise it installs the latest version at the time of execution.

e.g.
```
cargo install --version 1.0.0 conventional_commits_next_version
```


## Unit Testing
The unit test suite has a number parameterised tests testing the Conventional Commits v1.0.0 format parsing, cargo can be used to setup and run all the unit tests.

```
cargo test
```


## End-to-End Testing
To ensure correctness as there are a variety of out of process dependencies the project has an End-to-End behaviour driven test suite using the behave framework (https://github.com/behave/behave).
To run the test suite you need to first build a binary, install Python3, install behave and then execute behave to run the behaviour driven test suite.

__Note - You can't use --release as the test suite uses `target/debug/conventional_commits_next_version`.__

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
