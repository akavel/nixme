{.experimental: "codeReordering".}
import sets
import streams

type
  # Path = string
  LocalStore* = ref object
    paths*: HashSet[string]
    # root: Path

using
  s: LocalStore

proc query_valid_paths*(s; paths: openArray[string]): seq[string] =
  for p in paths:
    if s.paths.contains(p):
      result.add(p)

proc create_directory*(s; path: string) =
  discard

proc create_file*(s; path: string, executable: bool, size: uint64, contents: Stream) =
  discard
