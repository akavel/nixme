# Package

version       = "0.1.0"
author        = "Mateusz Czapli\xC5\x84ski"
description   = "Nix minimal effector"
license       = "LGPL-2.1"
srcDir        = "src"
installExt    = @["nim"]
bin           = @["nixme"]


# Dependencies

requires "nim >= 0.19.0"
requires "transcript"
