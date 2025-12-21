use std::{
    collections::HashMap,
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

use brotli_decompressor::Decompressor;
use serde::de::Error;

use crate::{CacheRef, Doc, SqliteDb};

pub type BlobId = usize;

/// Access the data in the files under `fs`.
///
/// These are either not compressed, or compressed with Brotli. The SQLite
/// database at `cache.db` tells us which it is.
#[derive(Debug)]
pub struct BlobStore {
    /// For a given ID, whether we have any data loaded yet, and whether it
    /// needs to be decompressed before use.
    cache: HashMap<BlobId, (Option<Vec<u8>>, bool)>,
    fs_root: PathBuf,
}

impl BlobStore {
    /// Create and somewhat lazily load the files necessary for the store.
    pub fn from_external_dir(sqlite_db: &SqliteDb, external_dir: &Path) -> rusqlite::Result<Self> {
        let cache = sqlite_db
            .all_data()?
            .into_iter()
            .map(|data| (data.row_id, (data.data, data.is_compressed)))
            .collect();
        Ok(Self {
            cache,
            fs_root: external_dir.join("fs"),
        })
    }

    /// Get the blob data.
    ///
    /// This will read it from the SQLite database at `cache.db`, or look it
    /// read, decompress and cache it from the `fs` directory in the external
    /// directory.
    pub fn blob(&mut self, idx: BlobId) -> io::Result<&[u8]> {
        let (blob, is_compressed) = self
            .cache
            .get_mut(&idx)
            .expect("invalid ID, cache was maybe wrongly pre-populated?");

        let blob = if let Some(blob) = blob {
            blob
        } else {
            *blob = Some(fs::read(self.fs_root.join(format!("{idx}")))?);
            blob.as_mut().unwrap()
        };

        if *is_compressed {
            *blob = decompress(&**blob)?;
            *is_compressed = false;
        }

        Ok(blob)
    }

    /// Get the slice of data corresponding to the given [`CacheRef`].
    pub fn data(&mut self, r: &CacheRef) -> io::Result<&[u8]> {
        let blob = self.blob(r.data_id)?;
        Ok(&blob[r.offset..r.offset + r.length])
    }

    /// Read a slice of data as a JSON [`Doc`].
    pub fn parse_doc(&mut self, r: &CacheRef) -> serde_json::Result<Doc> {
        let data = self.data(r).map_err(serde_json::Error::custom)?;
        serde_json::from_slice(data)
    }
}

/// Decompress a Reader using Brotli.
fn decompress(reader: impl Read) -> io::Result<Vec<u8>> {
    // let reader = BufReader::new(reader);

    let mut decompressed = Vec::new();
    let mut decompressor = Decompressor::new(reader, 1024 * 1024);
    decompressor.read_to_end(&mut decompressed)?;

    Ok(decompressed)
}
