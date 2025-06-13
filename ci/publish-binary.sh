#!/usr/bin/env sh

set -o errexit
set -o xtrace

if [ "$#" -ne 1 ]; then
	echo "Usage: $0 RELEASE_TAG"
	echo "$#"
	exit 1
fi

RELEASE="$1"

target="x86_64-unknown-linux-musl"
tar -czvf "${target}.tar.gz" -C "target/${target}/release" "conventional_commits_next_version"
gh release upload "${RELEASE}" "${target}.tar.gz"
rm "${target}.tar.gz"
