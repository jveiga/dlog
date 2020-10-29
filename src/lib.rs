#[derive(Debug, Default, Clone, PartialEq)]
pub struct Record {
    pub value: Vec<u8>,
    pub offset: Option<usize>,
}

#[derive(Debug, Default)]
pub struct Log {
    records: Vec<Record>,
}

impl Log {
    pub fn new() -> Self {
        Log::default()
    }

    pub fn append(&mut self, mut r: Record) -> Option<usize> {
        let offset = self.records.len();
        r.offset = Some(offset);
        self.records.push(r);

        Some(offset)
    }

    pub fn read(&self, offset: usize) -> Option<Record> {
        self.records.get(offset).map(|r| r.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_append() {
        let mut log = Log::new();
        let r = Record {
            value: vec![0u8; 42],
            offset: Some(2),
        };

        let res = log.append(r);

        assert!(res.is_some());
        assert_eq!(res.unwrap(), 0);
        assert_eq!(log.records.len(), 1);
    }

    #[test]
    fn test_get() {
        let mut log = Log::new();
        let r = Record {
            value: vec![0u8; 42],
            offset: Some(2),
        };
        let found = Record {
            value: r.value.clone(),
            offset: Some(0),
        };

        let _ = log.append(r);
        let res = log.read(0);

        assert!(res.is_some());
        assert_eq!(res.unwrap(), found);
        assert_eq!(log.records.len(), 1);
    }
}
