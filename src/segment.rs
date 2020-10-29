use std::fs::OpenOptions;

use crate::{
    config::Config,
    index::{Index, IndexRead},
    store::{Store, StoreWriteStatus},
};

use crate::{proto, Record};

use prost::Message;

// TODO
type BError = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct Segment {
    store: Store,
    index: Index,
    base_offset: u64,
    next_offset: u64,
    config: Config,
}

impl Segment {
    pub fn new(dir: &str, base_offset: u64, config: Config) -> std::io::Result<Self> {
        let store = Self::new_store(dir)?;
        let (index, next_offset) = Self::new_index(dir)?;
        let next_offset = next_offset + base_offset;

        Ok(Segment {
            store,
            config,
            index,
            base_offset,
            next_offset,
        })
    }

    #[tracing::instrument]
    pub fn append(&mut self, mut r: Record) -> Result<u64, BError> {
        let current_offset = self.next_offset;
        r.offset = current_offset;

        let mut rec = proto::Record::default();
        rec.value = r.value.clone();
        rec.offset = r.offset;
        let mut b = Vec::new();

        rec.encode(&mut b)?;

        let StoreWriteStatus {
            position,
            bytes_read: _,
        } = self.store.append(&b)?;

        self.index.write(
            (self.next_offset - self.base_offset) as u32,
            position as usize,
        )?;
        self.next_offset += 1;

        Ok(current_offset)
    }

    #[tracing::instrument]
    pub fn read(&mut self, offset: u64) -> Result<Record, BError> {
        let res = self.index.read((offset - self.base_offset) as i64)?;
        let buf = self.store.read(res.pos)?;
        let buf: &[u8] = buf.as_slice();
        let proto::Record { value, offset } = proto::Record::decode(buf)?;

        Ok(Record { value, offset })
    }

    fn new_store(dir: &str) -> std::io::Result<Store> {
        let path = format!("{}/store.store", dir);
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        Ok(Store::new(f)?)
    }

    pub fn is_maxed(&self) -> bool {
        self.store.size() >= self.config.segment.max_store_bytes
            || self.index.size() >= self.config.segment.max_index_bytes
    }

    pub fn remove(&mut self) -> std::io::Result<()> {
        self.close().unwrap();

        Ok(())
    }

    fn close(&mut self) -> Result<(), BError> {
        self.index.close()?;

        self.store.close()?;

        Ok(())
    }

    fn new_index(dir: &str) -> std::io::Result<(Index, u64)> {
        let path = format!("{}/index.index", dir);

        let idx = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        let index = Index::new(idx)?;
        let next_offset = if let Ok(IndexRead { out: _, pos }) = index.read(-1) {
            pos + 1
        } else {
            0
        };
        Ok((index, next_offset))
    }
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use std::fs::{File, OpenOptions};
    use std::path::PathBuf;

    use super::*;
    use crate::{Config, ENTWIDTH};

    use temp_testdir::TempDir;

    #[test]
    fn test_new() -> std::io::Result<()> {
        let temp = TempDir::default();
        let file_path = PathBuf::from(temp.as_ref());
        let c = Config::default();

        assert!(Segment::new(file_path.to_str().unwrap(), 0, c).is_ok());

        Ok(())
    }

    #[test]
    fn test_append() -> std::io::Result<()> {
        let temp = TempDir::default();
        let file_path = PathBuf::from(temp.as_ref());
        let c = Config::default();
        let mut seg = Segment::new(file_path.to_str().unwrap(), 0, c)?;
        let r = Record {
            value: "abcd".chars().map(|c| c as u8).collect(),
            offset: 0,
        };

        let offset = dbg!(seg.append(r.clone()).unwrap());

        assert_eq!(offset, 0);

        let other_record = dbg!(seg.read(offset).unwrap());

        assert_eq!(r, other_record);

        Ok(())
    }
}
