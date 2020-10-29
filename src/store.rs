use std::fs::File;
use std::io::Write;
use std::os::unix::fs::FileExt;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

const LENWIDTH: u64 = 8;

#[derive(Debug)]
pub struct Store {
    f: File,
    size: u64,
}

#[derive(Debug)]
pub struct StoreWriteStatus {
    pub bytes_read: u64,
    pub position: u64,
}

impl Store {
    pub fn new(f: File) -> std::io::Result<Self> {
        let meta = f.metadata()?;
        if meta.is_dir() {
            panic!("not supposed to use directory");
        }
        let size = meta.len();

        Ok(Self { f, size })
    }

    pub fn append(&mut self, b: &[u8]) -> std::io::Result<StoreWriteStatus> {
        let position = self.size;

        let mut bytes = vec![];
        bytes.write_u64::<BigEndian>(b.len() as u64).unwrap();

        self.f.write(&bytes)?;

        let bytes_read = self.f.write(b)? as u64;
        self.f.flush()?;

        self.size += bytes_read;

        Ok(StoreWriteStatus {
            bytes_read: bytes_read + LENWIDTH,
            position,
        })
    }

    pub fn read_at(&mut self, buf: &mut [u8], offset: u64) -> std::io::Result<()> {
        self.f.flush()?;

        self.f.read_at(buf, offset)?;

        Ok(())
    }

    pub fn read(&mut self, pos: u64) -> std::io::Result<Vec<u8>> {
        self.f.flush()?;

        let mut buf: [u8; 8] = [0u8; 8];
        self.f.read_at(&mut buf, pos)?;

        let record_size: u64 = unsafe { std::mem::transmute::<[u8; 8], u64>(buf) }.to_be();

        let mut buf: Vec<u8> = vec![0u8; record_size as usize];
        self.f.read_at(&mut buf[..], pos + LENWIDTH)?;

        Ok(buf.to_owned().to_vec())
    }

    #[inline(always)]
    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn close(&mut self) -> std::io::Result<()> {
        self.f.flush()?;

        Ok(())
    }
}

impl Drop for Store {
    fn drop(&mut self) {
        self.f.flush().map(drop).map_err(drop).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::default::Default;
    use std::fs::OpenOptions;
    use std::path::PathBuf;

    use super::*;
    use crate::{Config, ENTWIDTH};

    // use arbitrary::{Arbitrary, Result, Unstructured};
    use proptest::proptest;
    use proptest_derive::Arbitrary;
    use temp_testdir::TempDir;

    #[derive(Arbitrary, Debug)]
    enum Action {
        Append(Vec<u8>),
        ReadAt(u64),
        Read(u64),
    }

    #[derive(Arbitrary, Debug)]
    struct Actions(Vec<Action>);

    proptest! {
        #[test]
        fn test_prop(actions: Actions) {
            let temp = TempDir::default();
            let mut file_path = PathBuf::from(temp.as_ref());
            let mut c = Config::default();
            c.segment.max_store_bytes = 1024;
            c.segment.max_index_bytes = (ENTWIDTH as u64) * 3;
            file_path.push("store.store");
            let f = File::create(file_path).unwrap();
            let mut store = Store::new(f).unwrap();
            let mut total_size : u64 = 0;
            for action in actions.0{
                match action{
                    Action::Append(v) => {
                        assert!(store.append(&v).is_ok());
                        let size = v.len() as u64;
                        total_size+=size;
                        assert_eq!((total_size) , store.size());
                    },
                    Action::Read(pos) =>{
                       match store.read(pos) {
                           _ => {},
                       };
                    },
                    Action::ReadAt(pos) if pos < store.size() => {
                        let mut v = Vec::with_capacity(1024);
                        assert!(store.read_at(&mut v, pos).is_ok());
                    },
                    _ => {},
                }
            }
        }
    }
}
