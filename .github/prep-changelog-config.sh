#!/bin/bash

# Remove "refs/tags/"
tag="${GITHUB_REF##*/}"
# Extract crate name
crate=$(echo ${tag} | cut -d "-" -f1)
# Semver portion follows after the ${crate}-v
tagPattern="${crate}-v(.+)"

echo ::group::Configuring changelog generator
jq '.tag_resolver.filter.pattern="'$tagPattern'" | .tag_resolver.transformer.pattern="'$tagPattern'" | .categories[].labels += ["'$crate'"]' \
  .github/changelog-base.json | tee .github/changelog-config.json
echo ::endgroup::