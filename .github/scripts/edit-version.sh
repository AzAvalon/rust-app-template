#!/usr/bin/env bash
# Called by semantic-release (prepareCmd) with the new version: writes it to
# Cargo.toml, Cargo.lock and the chart. .releaserc commits these files.
set -euo pipefail

VERSION="${1:?Usage: $0 <version>}"
echo "Editing files with given version: ${VERSION}"

# cargo-set-version (cargo-edit) is installed by the release workflow.
# Updates the package version in Cargo.toml and its Cargo.lock entry.
cargo set-version "${VERSION}"

# Edit Helm chart
yq -i ".image.tag = \"${VERSION}\"" ./chart/values.yaml
yq -i ".version = \"${VERSION}\"" ./chart/Chart.yaml
yq -i ".appVersion = \"${VERSION}\"" ./chart/Chart.yaml

# Edit Helm chart doc
helm-docs
