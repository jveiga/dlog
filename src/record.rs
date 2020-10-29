pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/log.rs"));
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Record {
    pub value: Vec<u8>,
    pub offset: u64,
}

#[derive(Debug, Default)]
pub struct Log {
    pub records: Vec<Record>,
}

impl Log {
    pub fn new() -> Self {
        Log::default()
    }

    pub fn append(&mut self, mut r: Record) -> Option<usize> {
        let offset = self.records.len();
        r.offset = offset as u64;
        self.records.push(r);

        Some(offset)
    }

    pub fn read(&self, offset: usize) -> Option<Record> {
        self.records.get(offset).map(|r| r.clone())
    }
}
