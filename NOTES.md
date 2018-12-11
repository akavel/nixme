
# Goal

**NixME — is a Nix minimal effector**

NixME aims to be a minimal binary that can receive precompiled Nix derivations
from `nix copy --to ssh://$MACHINE`, and unpack them to `/nix/store/...` as
appropriate.

# Steps

 - A `nix copy --to ssh://$MACHINE` calls `LegacySSHStore` class in nix; this
   then calls `nix-store --serve` on target machine.
   **We must capture the protocol that ensues, to be able to reproduce it
   later.**
 - To do the above, I want to build a minimal QEMU VM with Nix preinstalled,
   and with a special SSH service that would have the `nix-store` command
   overridden to capture stdin & stdout to a file (with `socat`, if easier?).
   - https://ww.telent.net/2017/10/20/nixos_again_declarative_vms_with_qemu


