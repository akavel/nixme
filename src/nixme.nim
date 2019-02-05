{.experimental: "codeReordering".}
import nixmepkg/serve

# GOAL:
#
# Nixme is intended to be used as a Nix replacement for a `nix copy` target.
#
# MILESTONES
# - First of all, be able to create real files in a specified directory tree on disk,
#   unpacking them from received NAR data.
# - Then, correctly report NARs already received, to optimize data transferred from `nix copy`.
# - Eventually, support GCing - this will probably require storing references and GC roots. Might
#   want to implement some parts of the Nix SQLite database at this point.
# - LATER: implement full Nix SQLite database.
proc main():
  var store = LocalStore(paths: ["/nix/store/g2yk54hifqlsjiha3szr4q3ccmdzyrdv-glibc-2.27"])
  serve(store, stdin, stdout)

when isMainModule:
  main()
