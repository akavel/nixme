{.experimental: "codeReordering".}
import unittest
import streams
import strutils
import nixmepkg/nix_stream
import nixmepkg/nar

test "parse simple.nar":
  var
    buf = newSeq[string]()
  let
    input = openFileStream("tests/nar-simple/simple.nar")
    nix = wrap_nix_stream(input)
    handler = new(Handler)
  handler.create_directory = proc(path: string) =
    # echo "dir $#" % path
    buf.add "dir $#\n" % path
  handler.create_file = proc(path: string, executable: bool, size: uint64, contents: Stream) =
    if executable:
      # echo "exec $# = $#" % [path, $size]
      buf.add "exec $# = $#" % [path, $size]
    else:
      # echo "file $# = $#" % [path, $size]
      buf.add "file $# = $#" % [path, $size]
    buf.add contents.readAll
    # echo "\n"
    # buf.add "\n"
  handler.create_symlink = proc(path: string, target: string) =
    # echo "link $# -> $#" % [path, target]
    buf.add "link $# -> $#\n" % [path, target]

  parse_nar(nix, handler)
  check(buf.join("\n") == """
dir 

dir foo

file foo/bar = 0

file foo/baz = 0

file foo/data = 77
lasjdöaxnasd
asdom 12398
ä"§Æẞ¢«»”alsd
zażółć gęślą jaźń

exec foo/script.sh = 17
echo hello world

file foo-x = 0

link ln-dir -> foo

link ln-file -> foo/script.sh

file qux = 0

dir zyx
""")

