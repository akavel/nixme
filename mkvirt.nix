{ pkgs ? (import <nixpkgs> {})
, stdenv ? pkgs.stdenv
, bash ? pkgs.bash
, qemu_kvm ? pkgs.qemu_kvm
, writeText ? pkgs.writeText
}:

# Based on/inspired by: https://ww.telent.net/2017/10/20/nixos_again_declarative_vms_with_qemu

let
  # result = stdenv.mkDerivation {};
  result = (iso builtins.currentSystem);

  iso = system: (
    import <nixpkgs/nixos/lib/eval-config.nix> {
      inherit system;
      modules = [
        <nixpkgs/nixos/modules/installer/cd-dvd/installation-cd-minimal.nix>
        ./modules/nixos-auto-install-service.nix
      ];
    }).config.system.build.isoImage;

  #firstRunScript = pkgs.writeScript "firstrun.sh" ''
  #  #!${bash}/bin/bash
  #  hda=$1
  #  size=$2
  #  iso=$(echo /etc/nixos-cdrom.iso/nixos-*-linux.iso)
  #  PATH=/run/current-system/sw/bin:$PATH
  #  ${qemu_kvm}/bin/qemu-img  create -f qcow2  $hda.tmp $size
  #  mkdir -p /tmp/keys
  #  cp ${pubkey} /tmp/keys/ssh.pub
  #  ${qemu_kvm}/bin/qemu-kvm \
  #    -display vnc=127.0.0.1:99 \
  #    -m 512 \
  #    -drive file=$hda.tmp,if=virtio \
  #    -drive file=fat:floppy:/tmp/keys,if=virtio,readonly \
  #    -drive file=$iso,media=cdrom,readonly \
  #    -boot order=d \
  #    -serial stdio > $hda.console.log
  #  if grep INSTALL_SUCCESSFUL $hda.console.log; then
  #    mv $hda.tmp $hda
  #  fi
  #'';

  #pubkey = writeText "guest-pubkey" "ssh-rsa AAAATEST=";

in result

