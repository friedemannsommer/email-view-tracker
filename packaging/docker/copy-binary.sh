#!/usr/bin/env bash

set -xeuo pipefail

PREFIX="EVT_"
BINARY_NAME="email-view-tracker"

case "${TARGETPLATFORM}" in
    "linux/arm64") ARTIFACT_DIR="${PREFIX}aarch64-unknown-linux-musl" ;;
    "linux/arm/v6") ARTIFACT_DIR="${PREFIX}arm-unknown-linux-musleabihf" ;;
    "linux/arm/v7") ARTIFACT_DIR="${PREFIX}armv7-unknown-linux-musleabihf" ;;
    "linux/amd64") ARTIFACT_DIR="${PREFIX}x86_64-unknown-linux-musl" ;;
    "linux/386") ARTIFACT_DIR="${PREFIX}i686-unknown-linux-musl" ;;
    *) exit 1 ;;
esac;

cp "/opt/evt/binaries/$ARTIFACT_DIR/$BINARY_NAME" "/usr/local/bin/$BINARY_NAME";
chmod ugo=rx "/usr/local/bin/$BINARY_NAME";
