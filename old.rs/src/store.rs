use std::collections::HashSet;

pub trait Store {
    // TODO(akavel): try to make it accept both &["foo"] and vec![String::from("foo")]. See however:
    // - https://stackoverflow.com/q/54225766
    // and optionally:
    // - https://github.com/rust-lang/rust/issues/22031
    // - https://stackoverflow.com/a/41180422
    // - https://stackoverflow.com/q/48734211
    // TODO(akavel): return an Iterator<String>
    fn query_valid_paths(&mut self, paths: &mut dyn Iterator<Item = &str>) -> Vec<String>;
}

pub struct ValidPathInfo {
    pub path: String,    // FIXME: Path path;
    pub deriver: String, // FIXME: Path deriver;
    pub nar_hash: Hash,
    pub references: Vec<String>, // FIXME: PathSet references;
    // FIXME: time_t registrationTime = 0;
    pub nar_size: u64, // FIXME: uint64_t narSize = 0; // 0 = unknown
    // FIXME: uint64_t id; // internal use only
    /// Whether the path is ultimately trusted, that is, it's a
    /// derivation output that was built locally.
    pub ultimate: bool, // FIXME: bool ultimate = false;

    pub sigs: HashSet<String>, // FIXME: StringSet sigs; // note: not necessarily verified

    /* If non-empty, an assertion that the path is content-addressed,
       i.e., that the store path is computed from a cryptographic hash
       of the contents of the path, plus some other bits of data like
       the "name" part of the path. Such a path doesn't need
       signatures, since we don't have to trust anybody's claim that
       the path is the output of a particular derivation. (In the
       extensional store model, we have to trust that the *contents*
       of an output path of a derivation were actually produced by
       that derivation. In the intensional model, we have to trust
       that a particular output path was produced by a derivation; the
       path then implies the contents.)

       Ideally, the content-addressability assertion would just be a
       Boolean, and the store path would be computed from
       ‘storePathToName(path)’, ‘narHash’ and ‘references’. However,
       1) we've accumulated several types of content-addressed paths
       over the years; and 2) fixed-output derivations support
       multiple hash algorithms and serialisation methods (flat file
       vs NAR). Thus, ‘ca’ has one of the following forms:

       * ‘text:sha256:<sha256 hash of file contents>’: For paths
         computed by makeTextPath() / addTextToStore().

       * ‘fixed:<r?>:<ht>:<h>’: For paths computed by
         makeFixedOutputPath() / addToStore().
    */
    pub ca: String, // FIXME: std::string ca;
}

pub enum Base {
    Base64,
    Base32,
    Base16,
    Base64NoPrefix,
    Base32NoPrefix,
    Base16NoPrefix,
}

pub enum Hash {
    Md5(u16),
    // Sha1(u32),
    Sha256(u32),
    Sha512(u64),
}

impl Hash {
    pub fn to_string(&self, base: Base) -> String {
        "FIXME".to_string()
    }
}
