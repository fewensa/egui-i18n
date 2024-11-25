#!/bin/bash
#


set -ex

BIN_PATH=$(cd "$(dirname "$0")"; pwd -P)
WORK_PATH=${BIN_PATH}/../


cargo build \
  --manifest-path=${WORK_PATH}/Cargo.toml

export RUST_LOG=trace
${WORK_PATH}/target/debug/egui-i18n-cli $@
