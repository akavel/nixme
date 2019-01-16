# based on: github.com/nixos/nix/.../tests/nar-access.*
{ pkgs ? (import <nixpkgs> {})
, stdenv ? pkgs.stdenv
}:

rec {
  a = stdenv.mkDerivation {
    name = "nar-simple-a";
    builder = builtins.toFile "builder.sh" ''
      source $stdenv/setup

      mkdir $out
      mkdir $out/foo
      touch $out/foo-x
      touch $out/foo/bar
      touch $out/foo/baz
      touch $out/qux
      mkdir $out/zyx

      cat >$out/foo/data <<EOF
      lasjdöaxnasd
      asdom 12398
      ä"§Æẞ¢«»”alsd
      zażółć gęślą jaźń
      EOF

      cat >$out/foo/script.sh <<EOF
      echo hello world
      EOF
      chmod a+x $out/foo/script.sh
    '';
  };
}
