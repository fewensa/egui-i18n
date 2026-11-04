#!/bin/bash
set -ex

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
WORK_PATH=${BIN_PATH}/../
EXAMPLE="${1:-classic}"

if [[ "$EXAMPLE" != "classic" && "$EXAMPLE" != "fluent" ]]; then
  echo "usage: $0 [classic|fluent] [args...]" >&2
  exit 1
fi
shift || true

cargo build \
  --manifest-path="${WORK_PATH}/Cargo.toml" \
  -p "egui-i18n-example-${EXAMPLE}"

export RUST_LOG=trace
"${WORK_PATH}/target/debug/egui-i18n-example-${EXAMPLE}" "$@"
