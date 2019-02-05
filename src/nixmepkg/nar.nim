{.experimental: "codeReordering".}
import strutils

using
  r: Stream
  h: Handler
  path: Path

proc parse_nar*(r, h) =
  r.expect "nix-archive-1" # NAR_VERSION_MAGIC
  parse_node(r, h, "")

proc parse_node(r, h, path) =
  r.expect "("
  r.expect "type"
  let typ = r.read_str_ascii(20)
  case typ:
    of "regular":   parse_file(r, h, path)
    of "directory": parse_directory(r, h, path)
    of "symlink":   parse_symlink(r, h, path)
    else:           raise "unexpected node type, should be 'regular'/'directory'/'symlink': '%s'" % other

proc parse_file(r, h, path) =
  var word = r.read_str_ascii(20)
  let executable = word == "executable"
  if executable:
    r.expect ""
    word = r.read_str_ascii(20)
  if word != "contents":
    raise "unexpected word, should be 'contents': %s" % word
  let (size, blob_stream) = r.read_blob()
  h.create_file(path, executable, size, blob_stream)
  r.expect ")"

proc parse_directory(r, h, path) =
  h.create_directory(path)
  var prev_name = ""
  while true:
    let word = r.read_str_ascii(20)
    case word:
      of ")":     return
      of "entry": # ok
      else:       raise "unexpected word in directory: '%s'" % word
    r.expect "("
    r.expect "name"
    let name = r.read_str_ascii(MAX_NAME)
    if name == "" or name == "." or name == ".." or name.contains('/') or name.contains('\0'):
      raise "node name contains invalid characters: '%s'" % name
    if name <= prev_name:
      raise "node name not sorted: '%s' <= '%s'" % (name, prev_name)
    r.expect "node"
    parse_node(r, h, path / name)
    prev_name = name
    r.expect ")"

proc parse_symlink(r, h, path) =
  r.expect "target"
  h.create_symlink(path, r.read_str_ascii(MAX_TARGET))
  r.expect ")"

