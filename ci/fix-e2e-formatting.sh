#!/usr/bin/env sh

set -o errexit
set -o xtrace

find "./conventional_commits_next_version/end-to-end-tests/features" -type f -name "*.py" | xargs -I {} autopep8 --in-place --aggressive --aggressive --max-line-length 120 "{}"
