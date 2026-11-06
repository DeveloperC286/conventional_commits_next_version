# Conventional Commits Next Version
[![crates.io](https://img.shields.io/crates/v/conventional_commits_next_version)](https://crates.io/crates/conventional_commits_next_version)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)
[![License](https://img.shields.io/badge/License-AGPLv3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)


A tooling and language agnostic utility to calculate the next semantic version based on the Conventional Commits since the prior version. Supports monorepos.


## Why use Conventional Commits Next Version?
 * __Tooling & Language Agnostic__ - For input command-line arguments are utilised, there is no inbuilt functionality that only parses tooling or language-specific files for input.
 * __No Dependencies__ - A binary download is provided, removing dependencies on downloading tools or interpreted languages.
 * __Monorepo Support__ - Monorepo usage is supported, only commits altering the specified directory are used.
 * __Not A Linter__ - Minor deviations from the Conventional Commits specification are still accepted as valid input, because this is not a linter (but we suggest you use [Conventional Commits Linter](https://gitlab.com/DeveloperC/conventional_commits_linter)).
 * __Flexible__ - Non-Conventional Commits are ignored when encountered, and the calculation continues.


- [Usage](#usage)
  - [Usage - Consecutive Mode](#usage-consecutive-mode)
  - [Usage - Batch Mode](#usage-batch-mode)
- [Examples](#examples)
  - [GitHub Actions](#github-actions)
  - [GitLab CI](#gitlab-ci)
- [Installation](#installation)
  - [Binary](#binary)
  - [Cargo](#cargo)
  - [Docker](#docker)


## Usage
Conventional Commits Next Version can either operate upon a range of Git commits in the repositories' history or on a commit message provided by standard in.
To provide a commit message by standard in, use a hyphen (`-`) as the final positional argument.
Otherwise, to specify the range of commits, provide a commit reference (sha, tag, etc.) as the final positional argument.
The range of commits starts exclusively from the commit specified till inclusively of `HEAD`.

Any commits which conform to the Conventional Commits v1.0.0 specification are used to calculate the next Semantic Versioning, based upon the initial semantic version provided via the argument `--from-version <version>`.

The only required arguments are `--from-version <version>` and a final positional argument specifying either the commit reference (sha, tag, etc.) to start the calculation from, or a hyphen (`-`) to read from standard in.

The next semantic version can be calculated using a variety of calculation modes.
Currently there are two modes: consecutive and batch mode.


### Usage - Consecutive Mode
In consecutive mode each Git commit in the Conventional Commits specification is applied to Semantic Versioning calculation in chronological order.

Consecutive mode is the default, so no additional flags or configuration needs supplied.

E.g.

```
git clone https://github.com/yargs/yargs.git
cd yargs
git checkout 6014e39bca3a1e8445aa0fb2a435f6181e344c45
RUST_LOG=trace conventional_commits_next_version --from-version 15.2.0 c36c571e4e15dfe26be1d919e4991fb6ab6ed9fd
```

Using the environment variable `RUST_LOG` we can enable more detailed logging, so we can see the logic of consecutive mode.

```
DEBUG conventional_commits_next_version_lib::commits::commit > "fix: address ambiguity between nargs of 1 and requiresArg (#1572)\n\n" matches a patch Semantic Versioning increment commit message.
```

From the logs we can see that the commit `a5edc328ecb3f90d1ba09cfe70a0040f68adf50a` has the Conventional Commits type of `fix`.
The fix type will cause the increment of the initial semantic version provided via `--from-version` from `15.2.0` to `15.2.1`.

```
DEBUG conventional_commits_next_version_lib::commits::commit > "feat(yargs-parser): introduce single-digit boolean aliases (#1576)\n\n" matches a minor Semantic Versioning increment commit message.
```

From the logs we can see that the commit `3af7f04cdbfcbd4b3f432aca5144d43f21958c39` has the Conventional Commits type of `feat`.
The feat type encountered will increment the minor semantic version from `15.2.1` to `15.3.0`.

```
DEBUG conventional_commits_next_version_lib::commits::commit > "feat: add usage for single-digit boolean aliases (#1580)\n\n" matches a minor Semantic Versioning increment commit message.
```

From the logs we can see that the commit `6014e39bca3a1e8445aa0fb2a435f6181e344c451` has the Conventional Commits type of `feat`.
The feat type will increment the minor semantic version from `15.3.0` to `15.4.0`.

There are no more Conventional Commits which will increment the Semantic Versioning, so the calculated Semantic Versioning is printed to standard out.

```
15.4.0
```


### Usage - Batch Mode
In batch mode the largest Semantic Versioning increment determined by the Conventional Commits type across all the commits is the only increment applied.

Batch mode can be selected via the `--calculation-mode "Batch"` argument.

E.g.

```
git clone https://github.com/yargs/yargs.git
cd yargs
git checkout 6014e39bca3a1e8445aa0fb2a435f6181e344c45
RUST_LOG=trace conventional_commits_next_version --calculation-mode "Batch" --from-version 15.2.0 c36c571e4e15dfe26be1d919e4991fb6ab6ed9fd
```

Using the environment variable `RUST_LOG` we can see more detailed logs, to see how batch mode behaves differently.

```
DEBUG conventional_commits_next_version_lib::commits::commit > "feat(yargs-parser): introduce single-digit boolean aliases (#1576)\n\n" matches a minor Semantic Versioning increment commit message.
DEBUG conventional_commits_next_version_lib::commits::commit > "feat: add usage for single-digit boolean aliases (#1580)\n\n" matches a minor Semantic Versioning increment commit message.
```

The largest increment is a minor Semantic Versioning increment, because of the two commits `3af7f04cdbfcbd4b3f432aca5144d43f21958c39` and `6014e39bca3a1e8445aa0fb2a435f6181e344c45` with `feat` as the Conventional Commits' type.
The minor Semantic Versioning increment increases the initial semantic version from `15.2.0` to `15.3.0`, which is then printed to standard out.

```
15.3.0
```

## Examples
### GitHub Actions
<!-- x-release-please-start-version -->
```yaml
conventional-commits-next-version-checking:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code.
        uses: actions/checkout@v4
        with:
          fetch-depth: 0
      - name: Get current version.
        id: current_version
        run: echo "version=$(grep "^version = \"[0-9][0-9]*.[0-9][0-9]*.[0-9][0-9]*\"$" "Cargo.toml" | cut -d '"' -f 2)" >> $GITHUB_OUTPUT
      - name: Get latest tag.
        id: latest_tag
        run: echo "tag=$(git describe --tags --abbrev=0)" >> $GITHUB_OUTPUT
      - name: Check current vs expected.
        run: |
          version="v7.1.1" && wget -O - "https://github.com/DeveloperC286/conventional_commits_next_version/releases/download/${version}/x86_64-unknown-linux-gnu.tar.gz" | tar xz --directory "/usr/bin/"
          conventional_commits_next_version --calculation-mode "Batch" --current-version "${{ steps.current_version.outputs.version }}" --from-version "${{ steps.latest_tag.outputs.tag }}" "${{ steps.latest_tag.outputs.tag }}"
```
<!-- x-release-please-end -->

### GitLab CI
<!-- x-release-please-start-version -->
```yaml
conventional-commits-next-version-checking:
    stage: conventional-commits-next-version-checking
    image: rust
    before_script:
        - cargo install conventional_commits_next_version --version ^7
    script:
        # Get current version.
        - current_version=$(grep "^version = \"[0-9][0-9]*.[0-9][0-9]*.[0-9][0-9]*\"$" "Cargo.toml" | cut -d '"' -f 2)
        # Get latest tag.
        - latest_tag=$(git describe --tags --abbrev=0)
        # Check current vs expected.
        - /usr/local/cargo/bin/conventional_commits_next_version --calculation-mode "Batch" --current-version "${current_version}" --from-version "${latest_tag}" "${latest_tag}"
    rules:
        - if: $CI_MERGE_REQUEST_ID
```
<!-- x-release-please-end -->


## Installation
### Binary
Statically linked compiled binaries are available for download.
Visit the releases page at [https://github.com/DeveloperC286/conventional_commits_next_version/releases](https://github.com/DeveloperC286/conventional_commits_next_version/releases) to see all the releases, the release notes contains links to binary downloads for various architectures.

If you do not trust the provided binaries another option is to compile your own and then make it available for remote download, so your CICD etc can then download it.

<!-- x-release-please-start-version -->
```sh
version="v7.1.1" && wget -O - "https://github.com/DeveloperC286/conventional_commits_next_version/releases/download/${version}/x86_64-unknown-linux-musl.tar.gz" | tar xz --directory "/usr/bin/"
```
<!-- x-release-please-end -->

### Cargo
Cargo is the Rust package manager, the `install` sub-command pulls from [crates.io](https://crates.io/crates/conventional_commits_next_version) and then compiles the binary locally, placing the compiled binary at `${HOME}/.cargo/bin/conventional_commits_next_version`.

```sh
cargo install conventional_commits_next_version
```

By default it installs the latest version at the time of execution.
You can specify a specific version to install using the `--version` argument.
For certain environments such as CICD etc you may want to pin the version.

E.g.

<!-- x-release-please-start-version -->
```sh
cargo install conventional_commits_next_version --version 7.1.1
```
<!-- x-release-please-end -->

See [https://doc.rust-lang.org/cargo/commands/cargo-install.html#install-options](https://doc.rust-lang.org/cargo/commands/cargo-install.html#install-options) for more detailed documentation.

### Docker
You can use the Docker image published to [ghcr.io/developerc286/conventional_commits_next_version](https://github.com/DeveloperC286/conventional_commits_next_version/pkgs/container/conventional_commits_next_version).

## Issues/Feature Requests
Report issues or request features on our [GitHub Issues](https://github.com/DeveloperC286/conventional_commits_next_version/issues).
