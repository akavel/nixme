{ pkgs ? (import <nixpkgs> {})
, stdenv ? pkgs.stdenv
, bash ? pkgs.bash
, qemu_kvm ? pkgs.qemu_kvm
, writeText ? pkgs.writeText
, writeScript ? pkgs.writeScript
, runCommand ? pkgs.runCommand
, openssh ? pkgs.openssh
}:

# Based on/inspired by:
# - https://serverfault.com/questions/344295/is-it-possible-to-run-sshd-as-a-normal-user
# - https://serverfault.com/questions/407497/how-do-i-configure-sshd-to-permit-a-single-command-without-giving-full-login-ac
# - http://www.terminalinflection.com/strace-stdin-stdout/

let
  result = startSSH;

  startSSH = writeScript "startSSH" ''
    #!${bash}/bin/bash
    set -xeuo pipefail
    port="$1"

    ${openssh}/bin/sshd -D -f ${sshdConfig} -p "$port"
  '';

  sshdConfig = writeText "sshd_config" ''
    UsePrivilegeSeparation no
    HostKey "${home}/.ssh/id_rsa"
    PidFile "${home}/var/run/sshd.pid"
    AuthorizedKeysFile "${home}/.ssh/id_rsa.pub"
    ForceCommand ${interceptor}
  '';

  interceptor = writeScript "interceptor" ''
    #!${bash}/bin/bash
    mkdir -p ${home}/var/log
    strace -f -o "${home}/var/log/interceptor.log" -e read,write -e read=0,1,2 -e write=0,1,2 nix-store --serve
  '';

  home = builtins.getEnv "HOME";

in result

