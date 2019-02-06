{.experimental: "codeReordering".}
import streams
import nar
import nix_stream

# TODO: implement LocalStore with SQLite
# TODO: LATER: implement logging
# TODO: LATER: improve error handling to more helpful

type
  Store* {.explain.} = concept s
    s.query_valid_paths(openArray[string]) is seq[string]

# Based on NIX/src/nix-store/nix-store.cc, opServe()
# Other references:
# - NIX/src/libstore/legacy-ssh-store.cc
proc serve*(store: Store; r, w: NixStream) =
  # Exchange initial greeting/handshake (magic numbers)
  r.expect(0x390c_9deb'u64)
  w.write(0x5452_eecb'u64)
  # Announce protocol version
  # TODO(akavel): use version 0x205
  w.write(0x204'u64)
  w.flush()
  discard r.read[uint64]() # client version
  while true:
    # FIXME(akavel): exit successfully on EOF
    let cmd = r.read[uint64]()
    case cmd:
      of 1: # Query Valid Paths
        discard r.read[bool]() # TODO[LATER]: implement `lock` handling
        discard r.read[bool]() # TODO[LATER]: implement `substitute` handling
        let
          paths = r.read_strings_ascii(100, 300)  # FIXME(akavel): use some correct max lengths here
          response = store.query_valid_paths(paths)
        w.write(response)
        w.flush()
      of 2: # Query Path Infos
        # TODO(akavel): do we need to implement this cmd, or is it ok to just fake it?
        discard r.read_strings_ascii(100, 300)  # paths  # FIXME(akavel): use some correct max lengths here
        w.write(0'u64)
        w.flush()
      of 4: # Import Paths
        while true:
          let next = r.read[uint64]()
          case next:
            of 0: break
            of 1: discard
            else: raise "input doesn't look like something created by 'nix-store --export'"
          parse_nar(r, store)
          # Magic number
          r.expect(0x4558_494e'u64)
          # FIXME(akavel): use some correct max lengths here
          const MAX_PATH = 255
          discard r.read_str_ascii(MAX_PATH) # path
          discard r.read_strings_ascii(100, MAX_PATH) # references
          discard r.read_str_ascii(MAX_PATH) # deriver
          # Ignore optional legacy signature.
          if r.read[uint64]() == 1:
            discard r.read_str_ascii(MAX_PATH)
          w.write(1'u64) # indicate success
          w.flush()

# TODO: LATER: create temporary GC root
# TODO: LATER: if !repair && isValidPath(info.path) { return; }
# TODO: deletePath(realPath);
# TODO: restorePath(realPath, wrapperSource);
# TODO: LATER: verify hash
# TODO: LATER: verify size
# TODO: LATER: autoGC();
# TODO: LATER: canonicalisePathMetaData(realPath, -1);
# TODO: LATER: optimisePath(realPath);
# TODO: registerValidPath(info);
