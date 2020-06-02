# Conventional Commits Next Version
[![pipeline status](https://img.shields.io/badge/Version-1.0.0-blue)](https://gitlab.com/DeveloperC/conventional_commits_next_version/commits/master) [![pipeline status](https://gitlab.com/DeveloperC/conventional_commits_next_version/badges/master/pipeline.svg)](https://gitlab.com/DeveloperC/conventional_commits_next_version/commits/master) [![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

conventional_commits_next_version is a utility to calculate the next Semantic Versioning 2.0.0 (https://semver.org/spec/v2.0.0.html) based upon the supplied version and the Git commit history in the Conventional Commits v1.0.0 format (https://www.conventionalcommits.org/en/v1.0.0/).

## Why?
conventional_commits_next_version was created due to a lack of tooling not specific to and integrated with Node/NPM.
Following the Unix philosophy of 'Make each program do one thing well.' combined with utilising command line arguments and standard out; conventional_commits_next_version is a versatile tool not tied to specific tooling or language.

## Installing
If you wish to simply download a binary you can either use cargo or pull the binary from GitLab.

### Cargo
```
cargo install conventional_commits_next_version
```

###
Update <VERSION> to the desired version you wish to download. The downloaded file is zipped, so make sure to unzip it and it produces a './conventional_commits_next_version' binary.

```
wget https://gitlab.com/DeveloperC/conventional_commits_next_version/-/jobs/artifacts/<VERSION>/download?job=building-release
unzip download\?job=building-release
```

## Running
conventional_commits_next_version reads all the commit messages from HEAD in the current directory until the commit specified by the argument '--from-commit'.


### Consecutive

### Batch

## CICD
conventional_commits_next_version was designed to be used in a CICD pipeline.
Rust compiles to statically linked binaries, so you can download the latest version for your architecture from https://gitlab.com/DeveloperC/conventional_commits_next_version/, without needing to download additional dependencies.

e.g. GitLab CI for Rust

```

```

## Building
The binary and library can be built via cargo.

```
cargo build
```

## Unit Testing
There are a number of unit tests in the project, they can be ran via cargo.

```
cargo test
```

## End-to-End Testing
To ensure correctness as there are a variety of out of process dependencies the project has an End-to-End test suite.
The End-to-End suite uses the behave framework (https://github.com/behave/behave).
To execute the tests you need to first build the binary, install behave and then execute behave.

```
cargo build
cd end-to-end-tests/
virtualenv -p python3 .venv
pip install -r requirements.txt
behave
```
