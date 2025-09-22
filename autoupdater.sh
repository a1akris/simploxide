#!/bin/bash

set -e

local_simplex_version() {
    grep "^| 0\." ./README.md | head -1 | awk -F '|' '{ print $4 }' | tr -d ' '
}

upstream_simplex_version() {
    git submodule init
    git submodule update --remote

    grep "^version:" ./simploxide-bindgen/simplex-chat/simplex-chat.cabal | awk '{ print $2 }' | tr -d ' '
}

api_changed() {
    if [ -n "$(git status ./simploxide-api-types | grep 'Changes not staged for commit')" ]; then
        return 0
    else
        return 1
    fi
}

format_readme_row() {
    echo "| $1              | $2                  | $3                  |"
}

prepend_readme_row() {
    sed -i "/^| -----.*/a $(format_readme_row $1 $2 $3)" ./README.md
}

bump_crate_versions() {
    local current_ver=`grep "^version =" ./simploxide-api-types/Cargo.toml | awk -F '=' '{ print $2 }' | tr -d ' "'`
    local major minor patch

    IFS='.' read -r major minor patch <<<"$current_ver"
    local next_ver="$major.$((minor + 1)).0"

    sed -i 's/^version = .*/version = "'"$next_ver"'"/' ./simploxide-api-types/Cargo.toml
    sed -i 's/^version = .*/version = "'"$next_ver"'"/' ./simploxide-client/Cargo.toml
    sed -i 's/^version = .*/version = "'"$next_ver"'"/' ./simploxide/Cargo.toml

    sed -i -E "s/(^simploxide-api-types.*version = \")[^\"]+/\1${next_ver}/" simploxide-client/Cargo.toml

    echo "$next_ver"
}

cargo_publish() {
    cd "$1"
    cargo publish
    cd ../
}

this_ver=$(local_simplex_version)
next_ver=$(upstream_simplex_version)

if [ "$this_ver" = "$next_ver" ]; then
    echo "Stable versions match($this_ver)... Nothing to do"
    exit 0
fi

echo "Updating $this_ver -> $next_ver..."
./autobinder.sh

git config user.email "automaintainer@noreply.org"
git config user.name "Maintainer Agent"

# TODO: This is a temporary var for debugging(publish crates only after all
# other commands succeeded). The publish could be done before git push once the
# job stabilizes
publish="false"
if api_changed; then
    new_simploxide_ver=$(bump_crate_versions)
    ./lint.sh
    prepend_readme_row $new_simploxide_ver $next_ver $next_ver
    git add .
    git commit -m "Autoupdate: $this_ver -> $next_ver (BREAKING CHANGE)"
    git tag -a $new_simploxide_ver
    publish="true"
else
    sed -i "s/$this_ver/$next_ver/" ./README.md
    git add .
    git commit -m "Autoupdate: $this_ver -> $next_ver"
fi

git push
git push --tags

if [ "$publish" = "true" ]; then
    cargo_publish ./simploxide-api-types
    cargo_publish ./simploxide-client
    cargo_publish ./simploxide
fi
