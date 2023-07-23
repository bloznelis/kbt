#!/bin/bash

# Creates a new release branch, tags it and pushes
# both branch and tag to origin.
#
# Prerequisites:
#   * gum

set -euo pipefail

# import versioning function
source $(dirname -- "$0")/increment-version.sh

current_branch=$(git branch --show-current)

if [ $current_branch == 'master' ]
then
  gum spin --show-output --title="Pulling latest changes..." git pull origin master

  current_tag=`git describe --abbrev=0 --tags`

  increment_part=$(gum choose "minor" "patch" "major" --header "Choose update type:")

  new_tag=$(increment_version $current_tag $increment_part)

  gum confirm "Old tag: $current_tag | new tag: $new_tag" \
    && sed -i "s|version = \"[0-9.]*\"$|version = \"$new_tag\"|" Cargo.toml \
    && git add Cargo.toml && git commit -m "Release kbt $new_tag" \
    && git tag -a $new_tag -m "kbt $new_tag" \
    && gum spin --show-output --title="Pushing master" git push \
    && gum spin --show-output --title="Pushing tag: $new_tag" git push origin $new_tag
    

else
  echo -e "Realeases can only be created from \033[1mmaster\033[0m branch"
fi
