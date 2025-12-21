use std::path::Path;

use heed::byteorder::LittleEndian;
use heed::types::{Bytes, Str, U32, U64};
use heed::{BytesDecode, BytesEncode, Database, EnvFlags, EnvOpenOptions, RoTxn, WithoutTls};

use crate::{AvailabilityId, NavigatorId, PlatformAvailabilityId};

/// The database in `index/data.mdb`.
pub struct Lmdb {
    /// A map from [`AvailabilityId`] to a list of [`PlatformAvailabilityId`].
    ///
    /// These can then be looked up in availability.index.
    pub availability: Database<CodeAvailabilityId, DecodePlatformAvailabilities>,
    /// A map from various internal IDs to a string identifier. There can be
    /// multiple IDs that map to the same identifier.
    ///
    /// Example identifiers:
    /// - objective-c/documentation/appclip/apactivationpayload
    /// - swift/documentation/appclip/apactivationpayloaderror/code/doesnotmatch
    /// - swift/documentation/appkit/nsaccessibility-swift.struct
    /// - objective-c/documentation/quartz/ikimagebrowserdropon
    ///
    /// Two lookups are required to go from ID to value.
    pub index: Database<CodeIndexId, Bytes>,
    /// The transaction needed for accesses.
    pub rtxn: RoTxn<'static, WithoutTls>,
}

impl Lmdb {
    pub fn from_external_dir(external_dir: &Path) -> Result<Self, heed::Error> {
        Self::open(external_dir.join("index"))
    }

    /// Open the LMDB databases.
    pub fn open(env_path: impl AsRef<Path>) -> Result<Self, heed::Error> {
        let env = unsafe {
            EnvOpenOptions::new()
                .read_txn_without_tls()
                .map_size(1024 * 1024 * 1024)
                .max_dbs(2)
                .max_readers(10)
                // No need to lock, the file is read-only, even by Xcode.
                .flags(EnvFlags::NO_LOCK | EnvFlags::READ_ONLY)
                .open(env_path)?
        };

        let rtxn = env.read_txn()?;

        // mdb_dump -s availability external/index/
        let availability = env
            .open_database(&rtxn, Some("availability"))?
            .expect("must have availability database");

        // mdb_dump -s index external/index/
        let index: Database<CodeIndexId, Bytes> = env
            .open_database(&rtxn, Some("index"))?
            .expect("must have index database");

        rtxn.commit()?; // TODO: Call this elsewhere?

        let rtxn = env.static_read_txn()?;

        Ok(Self {
            availability,
            index,
            rtxn,
        })
    }

    /// Find the data associated with the given identifier.
    pub fn url_from_navigator_id(
        &self,
        navigator_item_id: NavigatorId,
    ) -> Result<Option<&str>, heed::Error> {
        let id = IndexId::NavigatorItemId(navigator_item_id);
        if let Some(res) = self.index.get(&self.rtxn, &id)? {
            Ok(Some(str::from_utf8(res).unwrap()))
        } else {
            Ok(None)
        }
    }

    /// Find data associated with an [`IndexId`].
    pub fn lookup_by_id(&self, id: IndexId<'_>) -> Result<Option<&str>, heed::Error> {
        if let IndexId::NavigatorItemId(navigator_item_id) = id {
            self.url_from_navigator_id(navigator_item_id)
        } else if let Some(res) = self.index.get(&self.rtxn, &id)? {
            let inner_id = CodeIndexId::bytes_decode(res).expect("res must be ID");
            let IndexId::NavigatorItemId(navigator_item_id) = inner_id else {
                panic!("Hex and Lang ID kinds must look up a NavigatorItemId");
            };
            self.url_from_navigator_id(navigator_item_id)
        } else {
            Ok(None)
        }
    }
}

/// Different kinds of IDs in the index database.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum IndexId<'a> {
    /// An ID from a navigator item, see [`NavigatorTree`] for details.
    ///
    /// Example values: `678512`, `615978`.
    ///
    /// [`NavigatorTree`]: crate::NavigatorTree
    NavigatorItemId(NavigatorId),
    /// It is unclear what this is used for!
    ///
    /// Example values: `"7dc314a9985e"`, `"4a0d2f054de1"`
    Hex(&'a str), // 6 hex bytes
    /// It is unclear what this is used for, but maybe some sort of deep
    /// linking?
    ///
    /// Example values:
    /// - `"Data-1a4mwlr"`
    /// - `"Data-qlavn"`
    /// - `"Objective-C-17gnmci"`
    /// - `"Objective-C-ofmfdm"`
    /// - `"Swift-6caqlo"`
    /// - `"Swift-1nxjnyc"`
    Lang(&'a str),
}

/// A encoder/decoder for [`IndexId`].
pub enum CodeIndexId {}

impl<'a> BytesEncode<'a> for CodeIndexId {
    type EItem = IndexId<'a>;

    fn bytes_encode(item: &'a Self::EItem) -> Result<std::borrow::Cow<'a, [u8]>, heed::BoxedError> {
        match item {
            IndexId::NavigatorItemId(num) => <U32<LittleEndian>>::bytes_encode(num),
            IndexId::Hex(hex) => Str::bytes_encode(hex),
            IndexId::Lang(val) => Str::bytes_encode(val),
        }
    }
}

impl<'a> BytesDecode<'a> for CodeIndexId {
    type DItem = IndexId<'a>;

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::DItem, heed::BoxedError> {
        if bytes.len() == 4 {
            return Ok(IndexId::NavigatorItemId(<U32<LittleEndian>>::bytes_decode(
                bytes,
            )?));
        }

        let data = str::from_utf8(bytes)?;

        let mut out = [0; 6];
        if let Ok(()) = hex::decode_to_slice(data, &mut out) {
            // NOTE: We could consider putting parsed hex data instead, but
            // that can be a bit difficult given
            Ok(IndexId::Hex(data))
        } else {
            Ok(IndexId::Lang(data))
        }
    }
}

/// An encoder and decoder for [`AvailabilityId`].
pub enum CodeAvailabilityId {}

impl<'a> BytesDecode<'a> for CodeAvailabilityId {
    type DItem = AvailabilityId;

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::DItem, heed::BoxedError> {
        Ok(AvailabilityId(<U64<LittleEndian>>::bytes_decode(bytes)?))
    }
}

impl<'a> BytesEncode<'a> for CodeAvailabilityId {
    type EItem = AvailabilityId;

    fn bytes_encode(item: &'a Self::EItem) -> Result<std::borrow::Cow<'a, [u8]>, heed::BoxedError> {
        <U64<LittleEndian>>::bytes_encode(&item.0)
    }
}

/// A decoder for platform availabilities (list of [`PlatformAvailabilityId`]).
pub enum DecodePlatformAvailabilities {}

impl<'a> BytesDecode<'a> for DecodePlatformAvailabilities {
    type DItem = &'a [PlatformAvailabilityId];

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::DItem, heed::BoxedError> {
        if cfg!(target_endian = "little") {
            Ok(bytemuck::try_cast_slice(bytes)?)
        } else {
            unimplemented!()
        }
    }
}
