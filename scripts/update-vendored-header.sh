#!/usr/bin/env bash
#
# Refresh the vendored copy of antithesis_instrumentation.h from the
# antithesis-sdk-cpp repository.
#
# To bump the vendored version, change CPP_SDK_VERSION below and re-run this
# script.

set -euo pipefail

CPP_SDK_VERSION="0.4.8"

URL="https://raw.githubusercontent.com/antithesishq/antithesis-sdk-cpp/${CPP_SDK_VERSION}/antithesis_instrumentation.h"
DEST="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/vendor/antithesis_instrumentation.h"

echo "fetching $URL"
curl -fsSL -o "$DEST" "$URL"

echo "vendored antithesis-sdk-cpp ${CPP_SDK_VERSION} -> ${DEST}"
