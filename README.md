# Conventional Commits Next Version
[![crates.io](https://img.shields.io/crates/v/conventional_commits_next_version)](https://crates.io/crates/conventional_commits_next_version) [![pipeline status](https://gitlab.com/DeveloperC/conventional_commits_next_version/badges/master/pipeline.svg)](https://gitlab.com/DeveloperC/conventional_commits_next_version/commits/master) [![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org) [![License: AGPL v3](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)


A tooling and language agnostic utility to calculate the next semantic version based on the Conventional Commits since the prior version. Supports monorepos.


## Why use Conventional Commits Next Version?
 * __Tooling & Language Agnostic__ - For input command-line arguments are utilised, there is no inbuilt functionality that only parses tooling or language-specific files for input.
 * __No Dependencies__ - A binary download is provided, removing dependencies on downloading tools or interpreted languages.
 * __Monorepo Support__ - Monorepo usage is supported, only commits altering the specified directory are used.
 * __Not A Linter__ - Minor deviations from the Conventional Commits specification are still accepted as valid input, because this is not a linter (but we suggest you use [Conventional Commits Linter](https://gitlab.com/DeveloperC/conventional_commits_linter)).
 * __Flexible__ - Non-Conventional Commits are ignored when encountered, and the calculation continues.


## Content
 * [Usage](#usage)
   + [Usage - Consecutive Mode](#usage-consecutive-mode)
   + [Usage - Batch Mode](#usage-batch-mode)
   + [Usage - Git Environment Variables](#usage-git-environment-variables)
   + [Usage - Monorepo](#usage-monorepo)
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
Conventional Commits Next Version uses the Git commits from a repository you specify using either the arguments `--from-commit-hash` or `--from-reference` or Git Environment Variables.
Any commits meeting the Conventional Commits v1.0.0 specification are then used to calculate the next semantic version, based upon the provided initial semantic version via the argument `--from-version`.
The next calculate version is printed to standard out.

If you specify the `--current-version` semantic version argument.
Then the provided version is asserted to be equal or larger than the calculated.
If the assertion is not met then it exits with a non zero exit code.

There are two modes of calculating the next semantic version, consecutive mode and batch mode.
By default Consecutive mode is used.


### Usage - Consecutive Mode
In consecutive mode each Git commit in the Conventional Commits specification is applied to Semantic Versioning calculation in chronological order.

Consecutive mode is the default, so no additional flags or configuration needs supplied.

e.g.

```
git clone https://github.com/yargs/yargs.git
cd yargs
git checkout 6014e39bca3a1e8445aa0fb2a435f6181e344c45
RUST_LOG=trace conventional_commits_next_version --from-commit-hash c36c571e4e15dfe26be1d919e4991fb6ab6ed9fd --from-version 15.2.0
```

Using the environment variable `RUST_LOG` we can enable more detailed logging, so we can see the logic of consecutive mode.

```
DEBUG conventional_commits_next_version::model::commits::commit > "fix: address ambiguity between nargs of 1 and requiresArg (#1572)\n\n" matches a patch Semantic Versioning increment commit message.
```

From the logs we can see that the commit `a5edc328ecb3f90d1ba09cfe70a0040f68adf50a` has the Conventional Commits type of `fix`.
The fix type will cause the increment of the initial Semantic Versioning provided via `--from-version` from `15.2.0` to `15.2.1`.

```
DEBUG conventional_commits_next_version::model::commits::commit > "feat(yargs-parser): introduce single-digit boolean aliases (#1576)\n\n" matches a minor Semantic Versioning increment commit message.
```

From the logs we can see that the commit `3af7f04cdbfcbd4b3f432aca5144d43f21958c39` has the Conventional Commits type of `feat`.
The feat type encountered will increment the minor Semantic Versioning from `15.2.1` to `15.3.0`.

```
DEBUG conventional_commits_next_version::model::commits::commit > "feat: add usage for single-digit boolean aliases (#1580)\n\n" matches a minor Semantic Versioning increment commit message.
```

From the logs we can see that the commit `6014e39bca3a1e8445aa0fb2a435f6181e344c451` has the Conventional Commits type of `feat`.
The feat type will increment the minor Semantic Versioning from `15.3.0` to `15.4.0`.

There are no more Conventional Commits which will increment the Semantic Versioning, so the calculated Semantic Versioning is printed to standard out.

```
15.4.0
```


### Usage - Batch Mode
In batch mode the single largest increment across all the Git commits in the Conventional Commits specification is the only increment applied to the Semantic Versioning.
Batch mode is useful for feature branches, if it has multiple fixes or a mixture of fixes and features being merged.

Batch mode is enabled via the `--batch-commits` flag.

e.g.

```
git clone https://github.com/yargs/yargs.git
cd yargs
git checkout 6014e39bca3a1e8445aa0fb2a435f6181e344c45
RUST_LOG=trace conventional_commits_next_version --from-commit-hash c36c571e4e15dfe26be1d919e4991fb6ab6ed9fd --from-version 15.2.0 --batch-commits
```

Using the environment variable `RUST_LOG` we can see more detailed logs, to see how batch mode behaves differently.

```
DEBUG conventional_commits_next_version::model::commits::commit > "feat(yargs-parser): introduce single-digit boolean aliases (#1576)\n\n" matches a minor Semantic Versioning increment commit message.
DEBUG conventional_commits_next_version::model::commits::commit > "feat: add usage for single-digit boolean aliases (#1580)\n\n" matches a minor Semantic Versioning increment commit message.
```

The largest increment is a minor Semantic Versioning increment, because of the two commits `3af7f04cdbfcbd4b3f432aca5144d43f21958c39` and `6014e39bca3a1e8445aa0fb2a435f6181e344c45` with `feat` as the Conventional Commits' type.
The minor Semantic Versioning increment increases the initial Semantic Versioning from `15.2.0` to `15.3.0`, which is then printed to standard out.

```
15.3.0
```


### Usage - Monorepo
The `--monorepo` argument supports the usage of monorepos.
Only commits which alter the specified entry from the argument are used when calculating the next semantic version.


### Usage - Git Environment Variables
When looking for a repository the Git environment variables are respected.
When `$GIT_DIR` is set, it takes precedence and Conventional Commits Next Version begins searching for a repository in the directory specified in `$GIT_DIR`.
When `$GIT_DIR` is not set, Conventional Commits Next Version searches for a repository beginning in the current directory.


### Usage - Logging
The crates `pretty_env_logger` and `log` are used to provide logging.
The environment variable `RUST_LOG` can be used to set the logging level.
See [https://crates.io/crates/pretty_env_logger](https://crates.io/crates/pretty_env_logger) for more detailed documentation.


## CICD Examples
### GitLab CI Rust Project Example
#### Via Cargo
See [Compiling via Cargo](#compiling-via-cargo) for more details about installing via Cargo.

__Note - This example downloads the latest `2.*` version.__

```
conventional-commits-next-version-checking:
    stage: conventional-commits-next-version-checking
    image: rust
    before_script:
        - cargo install conventional_commits_next_version --version ^2
    script:
        # Get current version and latest tag.
        - CURRENT_VERSION=`grep '^version = "[0-9][0-9]*.[0-9][0-9]*.[0-9][0-9]*"$' Cargo.toml | cut -d '"' -f 2`
        # Get latest tag.
        - LATEST_TAG=`git describe --tags --abbrev=0`
        # Check latest tag is in semantic versioning.
        - echo $LATEST_TAG | grep "^[0-9][0-9]*.[0-9][0-9]*.[0-9][0-9]*$"
        # Check current vs expected.
        - /usr/local/cargo/bin/conventional_commits_next_version --batch-commits --from-reference $LATEST_TAG --from-version $LATEST_TAG --current-version $CURRENT_VERSION
    rules:
        - if: $CI_MERGE_REQUEST_ID
```


#### Via Binary Download
See [Downloading Binary](#downloading-binary) for more details about Binary downloads.

__Note - This example downloads version `2.0.0`.__

```
conventional-commits-next-version-checking:
    stage: conventional-commits-next-version-checking
    image: rust
    before_script:
        - wget -q -O tmp.zip "https://gitlab.com/DeveloperC/conventional_commits_next_version/-/jobs/artifacts/2.0.0/download?job=release-binary-compiling-x86_64-linux-musl" && unzip tmp.zip && rm tmp.zip
    script:
        # Get current version and latest tag.
        - CURRENT_VERSION=`grep '^version = "[0-9][0-9]*.[0-9][0-9]*.[0-9][0-9]*"$' Cargo.toml | cut -d '"' -f 2`
        # Get latest tag.
        - LATEST_TAG=`git describe --tags --abbrev=0`
        # Check latest tag is in semantic versioning.
        - echo $LATEST_TAG | grep "^[0-9][0-9]*.[0-9][0-9]*.[0-9][0-9]*$"
        # Check current vs expected.
        - ./conventional_commits_next_version --batch-commits --from-reference $LATEST_TAG --from-version $LATEST_TAG --current-version $CURRENT_VERSION
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
Cargo is the Rust package manager, the `install` sub-command pulls from [crates.io](https://crates.io/crates/conventional_commits_next_version) and then compiles the binary locally, placing the compiled binary at `$HOME/.cargo/bin/conventional_commits_next_version`.

```
cargo install conventional_commits_next_version
```

By default it installs the latest version at the time of execution.
You can specify a specific version to install using the `--version` argument.
For certain environments such as CICD etc you may want to pin the version.

e.g.

```
cargo install conventional_commits_next_version --version 2.0.0
```

Rather than pinning to a specific version you can specify the major or minor version.

e.g.

```
cargo install conventional_commits_next_version --version ^2
```

Will download the latest `2.*` release whether that is `2.0.7` or `2.6.0`.


## Unit Testing
The unit test suite has several parameterised tests testing the Conventional Commits v1.0.0 format parsing.
Cargo is used to set up and run all the unit tests.

```
cargo test
```


## End-to-End Testing
To ensure correctness as there are various out of process dependencies, the project has an End-to-End behaviour driven test suite using the Behave framework (https://github.com/behave/behave).

To run the test suite you need to
 - Compile the Convention Commits Next Version binary.
 - Install Python3.
 - Install Behave.
 - Execute Behave.

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
