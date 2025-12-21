use std::path::Path;

use rusqlite::{Connection, OptionalExtension, Result, Row};

use crate::BlobId;

#[derive(Debug)]
pub struct BlobData {
    pub row_id: BlobId,
    pub data: Option<Vec<u8>>,
    pub is_compressed: bool,
}

#[derive(Debug)]
pub struct CacheRef {
    pub row_id: usize,
    pub uuid: String,
    pub data_id: BlobId,
    pub offset: usize,
    pub length: usize,
}

impl CacheRef {
    fn from_row(row: &Row<'_>) -> Result<Self> {
        Ok(Self {
            row_id: row.get(0)?,
            uuid: row.get(1)?,
            data_id: row.get(2)?,
            offset: row.get(3)?,
            length: row.get(4)?,
        })
    }
}

pub struct SqliteDb {
    conn: Connection,
}

impl SqliteDb {
    pub fn from_external_dir(external_dir: &Path) -> Result<Self> {
        Ok(Self {
            conn: Connection::open(external_dir.join("cache.db"))?,
        })
    }

    /// A UUID from either [`url_to_uuid`][crate::url_to_uuid] or
    /// [`TxtItem::uuid`][crate::TxtItem::uuid].
    pub fn get_ref(&self, uuid: &str) -> Result<Option<CacheRef>> {
        let mut stmt = self.conn.prepare(
            "SELECT row_id, uuid, data_id, offset, length FROM main.refs WHERE uuid = ?",
        )?;
        stmt.query_one([uuid], CacheRef::from_row).optional()
    }

    pub fn all_data(&self) -> Result<Vec<BlobData>> {
        let mut stmt = self
            .conn
            .prepare("SELECT row_id, data, is_compressed FROM main.data")?;
        let iter = stmt.query_map([], |row| {
            Ok(BlobData {
                row_id: row.get(0)?,
                data: row.get(1)?,
                is_compressed: row.get(2)?,
            })
        })?;
        Ok(iter.flatten().collect::<Vec<_>>())
    }

    pub fn all_refs(&self) -> Result<Vec<CacheRef>> {
        let mut stmt = self
            .conn
            .prepare("SELECT row_id, uuid, data_id, offset, length FROM main.refs")?;
        let iter = stmt.query_map([], CacheRef::from_row)?;
        Ok(iter.flatten().collect::<Vec<_>>())
    }
}
