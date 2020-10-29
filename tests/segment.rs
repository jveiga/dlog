use std::default::Default;
use std::path::PathBuf;

use dlog::*;

use temp_testdir::TempDir;
#[test]
fn test_something() {
    let temp = TempDir::default();
    let file_path = PathBuf::from(temp.as_ref());
    let mut c = Config::default();
    c.segment.max_store_bytes = 1024;
    c.segment.max_index_bytes = (ENTWIDTH as u64) * 3;
    let _segment = Segment::new(&file_path.to_str().unwrap(), 100, c).unwrap();
}
