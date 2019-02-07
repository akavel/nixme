{.experimental: "codeReordering".}
import strutils
import nix_stream
import streams
import ospaths

type
  Handler* = ref object
    create_directory*: proc(path: string)
    create_file*: proc(path: string, executable: bool, size: uint64, contents: Stream)
    create_symlink*: proc(path: string, target: string)

using
  r: NixStream
  h: Handler
  path: string

proc parse_nar*(r, h) =
  r.expect "nix-archive-1" # NAR_VERSION_MAGIC
  parse_node(r, h, "")

proc parse_directory(r, h, path)  # forward declaration seems required (because of recursive dependency?)

proc parse_node(r, h, path) =
  r.expect "("
  r.expect "type"
  case (let typ = r.read_str_ascii(20); typ):
    of "regular":   parse_file(r, h, path)
    of "directory": parse_directory(r, h, path)
    of "symlink":   parse_symlink(r, h, path)
    else:           raise newException(ProtocolError, "unexpected node type, should be 'regular'/'directory'/'symlink': '$#'" % typ)

proc parse_file(r, h, path) =
  let executable = case (let word = r.read_str_ascii(20); word)
    of "executable":
      r.expect ""
      r.expect "contents"
      true
    of "contents":
      false
    else:
      raise newException(ProtocolError, "unexpected word, should be 'contents'/'executable', got: '$#'" % word)
  let (size, blob_stream) = r.read_blob()
  h.create_file(path, executable, size, blob_stream)
  r.expect ")"

proc parse_directory(r, h, path) =
  h.create_directory(path)
  var prev_name = ""
  while true:
    case (let word = r.read_str_ascii(20); word):
      of ")":     return
      of "entry": discard
      else:       raise newException(ProtocolError, "unexpected word in directory: '$#'" % word)
    r.expect "("
    r.expect "name"
    let name = r.read_str_ascii(MAX_NAME)
    if name == "" or name == "." or name == ".." or name.contains('/') or name.contains('\0'):
      raise newException(ProtocolError, "node name contains invalid characters: '$#'" % name)
    if name <= prev_name:
      raise newException(ProtocolError, "node name not sorted: '$#' <= '$#'" % [name, prev_name])
    r.expect "node"
    let subpath = path / name
    parse_node(r, h, subpath)
    # parse_node(r, h, path / name)
    prev_name = name
    r.expect ")"

proc parse_symlink(r, h, path) =
  r.expect "target"
  h.create_symlink(path, r.read_str_ascii(MAX_TARGET))
  r.expect ")"

const
  MAX_TARGET = 255  # FIXME(akavel): arbitrary value
  MAX_NAME = 255    # FIXME(akavel): arbitrary value
