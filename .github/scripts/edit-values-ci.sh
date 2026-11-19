#!/usr/bin/env bash
# Called by the integration tests with the docker tag built for the PR:
# the kind cluster deploys that image.
set -euo pipefail

TAG="${1:?Usage: $0 <docker-tag>}"
echo "Setting docker tag: ${TAG}"

yq -i ".image.tag = \"${TAG}\"" .github/values.ci.yaml
