use itertools::join;
use rusqlite::{self as sql, types::ToSql};
use std::collections::HashSet;
use crate::err::Result;

use crate::store::{self, Base};

pub struct LocalStore {
    pub paths: HashSet<String>,
}

impl LocalStore {
    fn new() -> Self {
        LocalStore {
            paths: HashSet::new(),
        }
    }
}

impl store::Store for LocalStore {
    fn query_valid_paths(&mut self, paths: &mut dyn Iterator<Item = &str>) -> Vec<String> {
        let mut result = Vec::new();
        for p in paths {
            // println!("{:#?}", &p);
            if self.paths.contains(p) {
                result.push(p.to_string());
            }
        }
        result
    }
}

// SQLite database interface
struct Database {
    conn: sql::Connection,
}

macro_rules! maybe {
    ($value:expr; if $condition:expr) => {
        if $condition {
            Some($value)
        } else {
            None
        }
    };
}

impl Database {
    fn open() -> Result<Self> {
        let db = Database {
            conn: sql::Connection::open_in_memory()?,
        };
        db.conn.execute_batch(include_str!("local_store.sql"))?;
        Ok(db)
    }

    fn update_path_info(&mut self, info: &store::ValidPathInfo) -> Result<()> {
        let nar_size = maybe!(info.nar_size as i64; if info.nar_size > 0);
        let ultimate = maybe!(1; if info.ultimate);
        let sigs = maybe!(join(&info.sigs, " "); if !info.sigs.is_empty());
        let ca = maybe!(&info.ca; if !info.ca.is_empty());
        let mut stmt = self.conn.prepare_cached("update ValidPaths set narSize = ?, hash = ?, ultimate = ?, sigs = ?, ca = ? where path = ?;")?;
        stmt.execute(&[
            &nar_size as &ToSql,
            &info.nar_hash.to_string(Base::Base16),
            &ultimate,
            &sigs,
            &ca,
            &info.path,
        ])?;
        Ok(())
    }
}
