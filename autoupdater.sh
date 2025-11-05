#!/bin/bash

set -euo pipefail

min_supported_simplex_version() {
    grep "^| 0\." ./README.md | head -1 | awk -F '|' '{ print $3 }' | tr -d ' '
}

max_supported_simplex_version() {
    grep "^| 0\." ./README.md | head -1 | awk -F '|' '{ print $4 }' | tr -d ' '
}

upstream_simplex_version() {
    git submodule init &>/dev/null
    git submodule update --remote &>/dev/null

    grep "^version:" ./simploxide-bindgen/simplex-chat/simplex-chat.cabal | awk '{ print $2 }' | tr -d ' '
}

api_changed() {
    git status ./simploxide-api-types | grep -q -s 'Changes not staged for commit'
}

format_readme_row() {
    echo "| $1              | $2                  | $3                  |"
}

prepend_readme_row() {
    local row
    row=$(format_readme_row "$1" "$2" "$3")
    sed -i "/^| -----.*/a ${row}" ./README.md
}

get_crate_version() {
    grep "^version =" "$1/Cargo.toml" | awk -F '=' '{ print $2 }' | tr -d ' "'
}

bump_crate() {
    local current_ver next_ver major minor patch
    current_ver=$(get_crate_version "$1")

    IFS='.' read -r major minor patch <<<"${current_ver}"
    next_ver="${major}.$((minor + 1)).0"

    sed -i 's/^version = .*/version = "'"${next_ver}"'"/' "$1/Cargo.toml"
    echo "${next_ver}"
}

bump_crate_versions() {
    new_api_types_ver=$(bump_crate simploxide-api-types)
    new_client_ver=$(bump_crate simploxide-client)
    # TODO: bump simploxide version separetely after it gets implemented
    # sed -i 's/^version = .*/version = "'"${new_client_ver}"'"/' ./simploxide/Cargo.toml
    sed -i -E "s/(^simploxide-api-types.*version = \")[^\"]+/\1${new_api_types_ver}/" simploxide-client/Cargo.toml

    echo "${new_client_ver}"
}

cargo_publish() {
    cd "$1"
    cargo publish
    cd ../
}

this_ver=$(max_supported_simplex_version)
next_ver=$(upstream_simplex_version)

if [[ "${this_ver}" == "${next_ver}" ]]; then
    echo "Stable versions match(${this_ver})... Nothing to do"
    exit 0
fi

echo "Updating ${this_ver} -> ${next_ver}..."
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
    prepend_readme_row "${new_simploxide_ver}" "${next_ver}" "${next_ver}"
    git add .
    git commit -m "Autoupdate: ${this_ver} -> ${next_ver} (BREAKING CHANGE)"
    git tag -a "v${new_simploxide_ver}" -m "v${new_simploxide_ver}"
    publish="true"
else
    min_supported_ver=$(min_supported_simplex_version)
    sed_modifier=""

    if [[ "${min_supported_ver}" == "${this_ver}" ]]; then
        sed_modifier="2"
    fi

    sed -i "s/${this_ver}/${next_ver}/${sed_modifier}" ./README.md
    git add .
    git commit -m "Autoupdate: ${this_ver} -> ${next_ver}"
fi

git push
git push --tags

if [[ "${publish}" == "true" ]]; then
    cargo_publish ./simploxide-api-types
    cargo_publish ./simploxide-client
    cargo_publish ./simploxide
fi
