#!/usr/bin/env bash
# update-aur.sh <version>
# Updates PKGBUILD and .SRCINFO for a new release, then pushes to AUR.
# Run from the repo root: aur/update-aur.sh 0.3.0
set -euo pipefail

VERSION="${1:?Usage: $0 <version>}"
ARCHIVE_URL="https://github.com/imix/rpncalc/releases/download/v${VERSION}/rpncalc-x86_64-unknown-linux-gnu.tar.gz"

echo "Fetching checksum for v${VERSION}..."
SHA256=$(curl -sL "${ARCHIVE_URL}" | sha256sum | awk '{print $1}')
echo "sha256: ${SHA256}"

cd "$(dirname "$0")"

# Update pkgver and sha256sums in PKGBUILD
sed -i "s/^pkgver=.*/pkgver=${VERSION}/" PKGBUILD
sed -i "s/^pkgrel=.*/pkgrel=1/" PKGBUILD
sed -i "s/sha256sums_x86_64=('[^']*')/sha256sums_x86_64=('${SHA256}')/" PKGBUILD

# Regenerate .SRCINFO
makepkg --printsrcinfo > .SRCINFO

echo "Updated PKGBUILD and .SRCINFO for v${VERSION}"
echo "Review the changes, then push to AUR:"
echo "  cd aur && git add PKGBUILD .SRCINFO && git commit -m 'Update to ${VERSION}' && git push"
