#!/usr/bin/env bash
set -euo pipefail

REPO="nbwood98/nhl-draft-simulator"
BINARY="nhl-draft-simulator"

detect_platform() {
  local os arch
  os="$(uname -s)"
  arch="$(uname -m)"

  case "$os" in
    Linux)
      case "$arch" in
        x86_64) echo "linux-amd64" ;;
        *) echo ""; return 1 ;;
      esac
      ;;
    Darwin)
      case "$arch" in
        arm64) echo "macos-arm64" ;;
        *) echo ""; return 1 ;;
      esac
      ;;
    *)
      echo ""; return 1
      ;;
  esac
}

main() {
  local platform asset_name download_url tag

  platform="$(detect_platform)" || {
    echo "Error: Unsupported platform ($(uname -s)/$(uname -m))." >&2
    echo "Supported: Linux x86_64, macOS arm64, Windows (manual download)." >&2
    exit 1
  }

  echo "Detected platform: ${platform}"

  tag="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
    | grep '"tag_name"' | head -1 | cut -d'"' -f4)"

  if [ -z "$tag" ]; then
    echo "Error: Could not determine latest release." >&2
    exit 1
  fi

  echo "Latest release: ${tag}"

  asset_name="${BINARY}-${platform}"
  download_url="https://github.com/${REPO}/releases/download/${tag}/${asset_name}"

  echo "Downloading ${asset_name}..."
  curl -fSL -o "${BINARY}" "${download_url}"
  chmod +x "${BINARY}"

  echo ""
  echo "✓ Downloaded ${BINARY} (${tag}) to current directory."
  echo "  Run it with: ./${BINARY}"
}

main
