use std::fs::File;
use std::io::Write;

use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use memmap::{MmapMut, MmapOptions};

const OFFWIDTH: usize = 4;
const POSWIDTH: usize = 8;
pub const ENTWIDTH: usize = OFFWIDTH + POSWIDTH;

#[derive(Debug)]
pub struct Index {
    f: File,
    mmap: MmapMut,
    size: usize,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct IndexRead {
    pub out: u32,
    pub pos: u64,
}

#[derive(Debug, PartialEq)]
pub enum IndexReadErr {
    Empty,
    EOF,
}

impl std::fmt::Display for IndexReadErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IndexReadErr::Empty => "empty",
                IndexReadErr::EOF => "end of file",
            },
        )
    }
}

impl std::error::Error for IndexReadErr {}

type IndexError<T> = Result<T, Box<dyn std::error::Error>>;

impl Index {
    pub fn new(f: File) -> std::io::Result<Self> {
        let meta = f.metadata()?;
        let size = meta.len() as usize;

        f.set_len(1024)?;

        // truncate file to a maximum config size?
        let mmap = unsafe { MmapOptions::new().len(1024).map_mut(&f)? };

        Ok(Index { f, mmap, size })
    }

    pub fn read(&self, idx: i64) -> Result<IndexRead, IndexReadErr> {
        if self.size == 0 {
            return Err(IndexReadErr::Empty);
        }

        let out: usize = if idx == -1 {
            (self.size as usize / ENTWIDTH) - 1
        } else {
            idx as usize
        } as usize;

        let pos_idx: usize = out * ENTWIDTH;
        if (self.size as usize) < pos_idx + ENTWIDTH {
            return Err(IndexReadErr::EOF);
        }

        let mut out_bytes = self.mmap.get(pos_idx..pos_idx + OFFWIDTH).unwrap();

        let out = out_bytes.read_u32::<BigEndian>().unwrap();

        let mut pos_bytes: &[u8] = self
            .mmap
            .get(pos_idx + OFFWIDTH..pos_idx + ENTWIDTH)
            .unwrap();

        let pos: u64 = pos_bytes.read_u64::<BigEndian>().unwrap();

        Ok(IndexRead { out, pos })
    }

    pub fn write(&mut self, off: u32, pos: usize) -> IndexError<()> {
        if self.mmap.len() < self.size + ENTWIDTH {
            return Err("file full?".into());
        }

        let mut off_bytes = [0u8; 4];
        // off_bytes.write_u32::<BigEndian>(off as u32)?;
        BigEndian::write_u32(&mut off_bytes, off);

        (&mut self.mmap[self.size..self.size + OFFWIDTH]).write_all(&off_bytes[0..4])?;

        let mut pos_bytes = vec![];
        pos_bytes.write_u64::<BigEndian>(pos as u64)?;

        (&mut self.mmap[self.size + OFFWIDTH..self.size + ENTWIDTH]).write_all(&pos_bytes[0..8])?;

        self.mmap.flush()?;
        self.size = self.size + ENTWIDTH;

        Ok(())
    }

    #[inline(always)]
    pub fn size(&self) -> u64 {
        self.size as u64
    }

    pub fn close(&mut self) -> std::io::Result<()> {
        self.f.flush()?;

        self.mmap.flush()?;

        self.f.set_len(self.size as u64)?;

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn new_temp_file(file_name: &str) -> Result<std::fs::File, std::io::Error> {
        use std::fs::OpenOptions;
        use std::path::PathBuf;
        use temp_testdir::TempDir;
        let temp = TempDir::default();
        let mut file_path = PathBuf::from(temp.as_ref());
        file_path.push(file_name);

        Ok(OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?)
    }

    #[test]
    fn test_new() -> TestResult {
        let f = new_temp_file("index")?;
        let index = Index::new(f)?;

        assert_eq!(0, index.size());

        Ok(())
    }

    #[test]
    fn test_read() -> TestResult {
        let f = new_temp_file("index")?;
        let mut index = Index::new(f)?;

        assert_eq!(0, index.size());

        let read_result = index.read(0);
        assert!(read_result.is_err());
        assert_eq!(IndexReadErr::Empty, read_result.unwrap_err());

        let write_result = index.write(1024, 0);
        assert_eq!((), write_result.unwrap());

        let write_result = index.read(0);
        assert_eq!(IndexRead { out: 1024, pos: 0 }, write_result.unwrap());

        assert_eq!(
            // pos + offset
            (std::mem::size_of::<u32>() + std::mem::size_of::<u64>()) as u64,
            index.size()
        );

        Ok(())
    }

    #[test]
    fn test_write() -> TestResult {
        let f = new_temp_file("index")?;
        let mut index = Index::new(f)?;

        assert_eq!(0, index.size());
        // TODO
        Ok(())
    }
}
