# Goal

**NixME — is a Nix minimal effector**

NixME aims to be a minimal binary that can receive precompiled Nix derivations
from `nix copy --to ssh://$MACHINE`, and unpack them to `/nix/store/...` as
appropriate.

# Implementation steps

 - ~~A `nix copy --to ssh://$MACHINE` calls `LegacySSHStore` class in nix; this
   then calls `nix-store --serve` on target machine.
   **We must capture the protocol that ensues, to be able to reproduce it
   later.**~~ *(done)*
 - Based on the dumps in testdata/, build test cases. The eventual Nixme app
   should correctly reproduce the communication from those dumps.

