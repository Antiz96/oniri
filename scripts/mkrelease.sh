#!/bin/bash
set -euo pipefail

# Check that we are on the main branch
if [ "$(git branch --show-current)" != "main" ]; then
	echo -e >&2 "ERROR: Current branch is not 'main'"
	exit 1
fi

# Check that `github-cli` is correctly authenticated
if ! gh auth status > /dev/null; then
	echo -e >&2 "ERROR: github-cli is not authenticated"
	exit 2
fi

# Pull repo and fetch tags
git pull
git fetch --tags

# Assign and check tag variables
release_tag="${1}"
latest_tag=$(git describe --tags --abbrev=0)

if [ -z "${release_tag}" ]; then
	echo -e >&2 "\nERROR: Release tag is empty\nUsage: ./scripts/mkrelease.sh X.Y.Z # where 'X.Y.Z' is the tag to create"
	exit 3
fi

echo -e "\nRelease tag = v${release_tag}\nLatest tag = ${latest_tag}\n"
read -rp "Confirm? [y/N] " answer

case "${answer}" in
	y|Y)
		echo -e "\nProceeding in 5 sec"
		sleep 5
	;;
	*)
		echo -e >&2 "\nAborting"
		exit 4
	;;
esac

# Build binary
rm -rf target/ && cargo build --release

# Bump version where necessary
sed_pattern="${latest_tag//./\\.}" # escape dots
sed -i "s/${sed_pattern#v}/${release_tag}/g" Cargo.toml doc/man/oniri.1.scd

# Update changelog
git-cliff -up CHANGELOG.md
sed -i "s|\[unreleased\]|\[v${release_tag}\](https://github.com/Antiz96/oniri/releases/tag/v${release_tag})\ -\ $(date '+%Y-%m-%d')|g" CHANGELOG.md

# Review changes
git diff
echo
read -rp "Confirm? [y/N] " answer

case "${answer}" in
	y|Y)
		echo -e "\nProceeding in 5 sec"
		sleep 5
	;;
	*)
		echo -e >&2 "\nAborting"
		exit 5
	;;
esac

# Create and push a signed commit
git add .
git commit -SD33FAA16B937F3B2 -m "chore(release): v${release_tag}"
git push

# Create and push a signed tag
git tag "v${release_tag}" -u D33FAA16B937F3B2 -m "v${release_tag}"
git push origin "v${release_tag}"

# Create release
echo -e "\nType (or paste) release notes, press ctrl+d when done\n"
gh release create "v${release_tag}" --title "v${release_tag}" --verify-tag -F -

# Download and sign auto-generated source tarball and checksum
gh release download "v${release_tag}" --archive tar.gz --clobber
gpg --local-user D33FAA16B937F3B2 --armor --detach-sign "oniri-${release_tag}.tar.gz"
sha256sum "oniri-${release_tag}.tar.gz" > "oniri-${release_tag}.tar.gz.sha256"
gpg --local-user D33FAA16B937F3B2 --armor --detach-sign "oniri-${release_tag}.tar.gz.sha256"

# Sign binary and checksum
mv target/release/oniri "target/release/oniri-${release_tag}-amd64"
gpg --local-user D33FAA16B937F3B2 --armor --detach-sign "target/release/oniri-${release_tag}-amd64"
sha256sum "target/release/oniri-${release_tag}-amd64" > "target/release/oniri-${release_tag}-amd64.sha256"
gpg --local-user D33FAA16B937F3B2 --armor --detach-sign "target/release/oniri-${release_tag}-amd64.sha256"

# Upload assets
gh release upload "v${release_tag}" \
	"oniri-${release_tag}.tar.gz.asc" \
	"oniri-${release_tag}.tar.gz.sha256" \
	"oniri-${release_tag}.tar.gz.sha256.asc" \
	"target/release/oniri-${release_tag}-amd64" \
	"target/release/oniri-${release_tag}-amd64.asc" \
	"target/release/oniri-${release_tag}-amd64.sha256" \
	"target/release/oniri-${release_tag}-amd64.sha256.asc"

# Cleanup
rm -rf "oniri-${release_tag}.tar.gz"* target/

