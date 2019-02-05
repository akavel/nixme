{.experimental: "codeReordering".}

# TODO: implement LocalStore with SQLite
# TODO: LATER: implement logging
# TODO: LATER: improve error handling to more helpful

# Based on NIX/src/nix-store/nix-store.cc, opServe()
# Other references:
# - NIX/src/libstore/legacy-ssh-store.cc
proc serve*(store: Store; r, w: Stream) =
  # Exchange initial greeting/handshake (magic numbers)
  r.expect(0x390c_9deb'u64)
  w.write(0x5452_eecb'u64)
  # Announce protocol version
  # TODO(akavel): use version 0x205
  w.write(0x204'u64)
  w.flush()
  let _client_version = r.read[uint64]()
  while true:
    # FIXME(akavel): exit successfully on EOF
    let cmd = r.read[uint64]()
    case cmd:
      of 1: # Query Valid Paths
        let
          _lock = r.read[bool]()                  # TODO[LATER]: implement `lock` handling
          _substitute = r.read[bool]()            # TODO[LATER]: implement `substitute` handling
          paths = r.read_strings_ascii(100, 300)  # FIXME(akavel): use some correct max lengths here
          response = store.query_valid_paths(paths)
        w.write(response)
        w.flush()
      of 2: # Query Path Infos
        # TODO(akavel): do we need to implement this cmd, or is it ok to just fake it?
        let _paths = r.read_strings_ascii(100, 300)  # FIXME(akavel): use some correct max lengths here
        w.write(0'u64)
        w.flush()
      of 4: # Import Paths
        while true:
          let next = r.read[uint64]()
          case next:
            of 0:
              break
            of 1:
              # ok
            else:
              raise "input doesn't look like something created by 'nix-store --export'"
          nar.parse(r, store)
          # Magic number
          r.expect(0x4558_494e'u64)
          # FIXME(akavel): use some correct max lengths here
          const MAX_PATH = 255
          let
            _path = r.read_str_ascii(MAX_PATH)
            _references = r.read_strings_ascii(100, MAX_PATH)
            _deriver = r.read_str_ascii(MAX_PATH)
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
