use failure;
use itertools::join;
use rusqlite::{self as sql, types::ToSql};
use std::collections::HashSet;

use crate::store::{self, Base};

pub type Result<T> = std::result::Result<T, failure::Error>;

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

impl Database {
    fn update_path_info(&mut self, info: &store::ValidPathInfo) -> Result<()> {
        let nar_size = if info.nar_size > 0 {
            Some(info.nar_size as i64)
        } else {
            None
        };
        let ultimate = if info.ultimate { Some(1) } else { None };
        let sigs = if !info.sigs.is_empty() {
            Some(join(&info.sigs, " "))
        } else {
            None
        };
        let ca = if !info.ca.is_empty() {
            Some(&info.ca)
        } else {
            None
        };
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
