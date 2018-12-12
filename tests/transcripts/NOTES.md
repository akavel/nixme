
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
 - To do the above, I want to build a special SSH service that would have the
   `nix-store` command overridden to capture stdin & stdout to a file (with
   `socat`, if easier?).
    - sample command for testing:

            $ nix copy --to ssh://localhost --from https://cache.nixos.org /nix/store/nkp4ck5scygkjk87nr6w61gb23li829m-hello-2.10

        Environment:

            $ nix build dumper.nix && ./result 2022                     # will start SSH server
            $ sudo `which socat` tcp-listen:22,fork tcp:localhost:2022  # will redirect port 22 to 2022
