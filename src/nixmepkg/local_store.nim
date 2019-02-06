{.experimental: "codeReordering".}
import sets

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

