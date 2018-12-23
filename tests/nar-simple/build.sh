# based on: github.com/nixos/nix/.../tests/nar-access.*
set -xeuo pipefail
storePath="$(nix-build nar-simple.nix -A a --no-out-link)"
nix-store --dump $storePath > simple.nar

