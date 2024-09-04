#!/usr/bin/env sh

set -o errexit
set -o xtrace

cd "conventional_commits_next_version/end-to-end-tests/"
behave
