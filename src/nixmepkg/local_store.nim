{.experimental: "codeReordering".}
import sets
import nar
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

proc nar_handler*(s): Handler =
  let h = new(Handler)
  h.create_directory = proc(path: string) =
    discard
  h.create_file = proc(path: string, executable: bool, size: uint64, contents: Stream) =
    discard
  h.create_symlink = proc(path: string; target: string) =
    discard
  return h
