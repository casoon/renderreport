#!/usr/bin/env sh
set -e

REPO="casoon/renderreport"
BIN="renderreport"
INSTALL_DIR="${RENDERREPORT_INSTALL_DIR:-/usr/local/bin}"

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Darwin)
    case "$ARCH" in
      arm64)  TARGET="aarch64-apple-darwin" ;;
      x86_64) TARGET="x86_64-apple-darwin" ;;
      *)      echo "Unsupported architecture: $ARCH" >&2; exit 1 ;;
    esac
    ;;
  Linux)
    case "$ARCH" in
      aarch64) TARGET="aarch64-unknown-linux-gnu" ;;
      x86_64)  TARGET="x86_64-unknown-linux-gnu" ;;
      *)       echo "Unsupported architecture: $ARCH" >&2; exit 1 ;;
    esac
    ;;
  *)
    echo "Unsupported OS: $OS" >&2
    echo "On Windows, download the .zip from https://github.com/${REPO}/releases" >&2
    exit 1
    ;;
esac

# Resolve version
if [ -z "$VERSION" ]; then
  VERSION="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed 's/.*"tag_name": *"v\([^"]*\)".*/\1/')"
fi

if [ -z "$VERSION" ]; then
  echo "Could not determine latest version" >&2
  exit 1
fi

ASSET="${BIN}-v${VERSION}-${TARGET}.tar.gz"
RELEASE_BASE_URL="${RENDERREPORT_RELEASE_BASE_URL:-https://github.com/${REPO}/releases/download/v${VERSION}}"

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

echo "Installing renderreport v${VERSION} (${TARGET})..."
curl -fsSL "${RELEASE_BASE_URL}/${ASSET}" -o "${TMP}/${ASSET}"
curl -fsSL "${RELEASE_BASE_URL}/${ASSET}.sha256" -o "${TMP}/${ASSET}.sha256"

EXPECTED_CHECKSUM="$(awk '{ print $1; exit }' "${TMP}/${ASSET}.sha256")"
if [ -z "$EXPECTED_CHECKSUM" ]; then
  echo "No checksum published for ${ASSET}" >&2
  exit 1
fi
if command -v sha256sum >/dev/null 2>&1; then
  ACTUAL_CHECKSUM="$(sha256sum "${TMP}/${ASSET}" | awk '{ print $1 }')"
elif command -v shasum >/dev/null 2>&1; then
  ACTUAL_CHECKSUM="$(shasum -a 256 "${TMP}/${ASSET}" | awk '{ print $1 }')"
else
  echo "No SHA-256 verification tool found" >&2
  exit 1
fi
if [ "$ACTUAL_CHECKSUM" != "$EXPECTED_CHECKSUM" ]; then
  echo "Checksum verification failed for ${ASSET}" >&2
  exit 1
fi

tar -xzf "${TMP}/${ASSET}" -C "$TMP"

if [ ! -w "$INSTALL_DIR" ]; then
  echo "Installing to ${INSTALL_DIR} (requires sudo)..."
  sudo install -m 755 "${TMP}/${BIN}" "${INSTALL_DIR}/${BIN}"
else
  install -m 755 "${TMP}/${BIN}" "${INSTALL_DIR}/${BIN}"
fi

echo "Installed: $(${INSTALL_DIR}/${BIN} --version)"
