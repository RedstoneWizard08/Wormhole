use crate::check_bytes;
use anyhow::Result;
use byteorder::{ReadBytesExt, WriteBytesExt};
use flate2::{read::GzDecoder, write::GzEncoder};
use std::{
    fs,
    io::{BufRead, Read, Write},
    path::PathBuf,
};

pub const CURRENT_VERSION: [u8; 3] = [0, 1, 0];

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type,
)]
pub struct IndexVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type)]
pub struct IndexFile {
    pub path: String,
    pub sha1: String,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Type, Default,
)]
pub struct PackIndex {
    pub ver: IndexVersion,
    pub files: Vec<IndexFile>,
}

impl Default for IndexVersion {
    fn default() -> Self {
        Self {
            major: CURRENT_VERSION[0],
            minor: CURRENT_VERSION[1],
            patch: CURRENT_VERSION[2],
        }
    }
}

impl PackIndex {
    pub fn read(path: impl Into<PathBuf>) -> Result<Self> {
        Self::parse(fs::read(path.into())?.as_slice())
    }

    pub fn parse(data: &[u8]) -> Result<Self> {
        let mut buf = Vec::new();

        GzDecoder::new(data).read_to_end(&mut buf)?;

        let mut data = buf.as_slice();

        check_bytes!(#read(4) data == b"INDX");

        let ver = IndexVersion {
            major: data.read_u8()?,
            minor: data.read_u8()?,
            patch: data.read_u8()?,
        };

        check_bytes!(#read(4) data == b"DATA");

        let mut files = Vec::new();

        while data.has_data_left()? {
            let mut file = Vec::new();

            data.read_until(0, &mut file)?;
            file.pop();

            let mut hash = Vec::new();

            data.read_until(0, &mut hash)?;
            hash.pop();

            files.push(IndexFile {
                path: String::from_utf8(file)?,
                sha1: String::from_utf8(hash)?,
            });
        }

        Ok(Self { files, ver })
    }

    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut data = GzEncoder::new(Vec::new(), Default::default());

        data.write_all(b"INDX")?;
        data.write_u8(self.ver.major)?;
        data.write_u8(self.ver.minor)?;
        data.write_u8(self.ver.patch)?;
        data.write_all(b"DATA")?;

        for file in &self.files {
            data.write_all(file.path.as_bytes())?;
            data.write_u8(0)?;
            data.write_all(file.sha1.as_bytes())?;
            data.write_u8(0)?;
        }

        Ok(data.finish()?)
    }
}

#[cfg(test)]
mod _test {
    use super::PackIndex;
    use flate2::write::GzEncoder;
    use std::io::Write;

    #[test]
    fn test_ser_de() {
        let mut v = Vec::new();

        v.extend_from_slice(b"INDX");
        v.push(0);
        v.push(1);
        v.push(0);
        v.extend_from_slice(b"DATA");
        v.extend_from_slice(b"Fsome_mod.jar\0");
        v.extend_from_slice(b"Hsome_sha1\0");
        v.extend_from_slice(b"Fsome_other_mod.jar\0");
        v.extend_from_slice(b"Hsome_other_sha1\0");

        let mut gz = GzEncoder::new(Vec::new(), Default::default());

        gz.write_all(v.as_slice()).unwrap();

        let data = gz.finish().unwrap();
        let idx = PackIndex::parse(data.as_slice()).unwrap();
        let enc = idx.encode().unwrap();

        assert_eq!(data, enc);
    }
}
