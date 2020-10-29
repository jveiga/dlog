use dlog::*;

#[test]
fn test_append() {
    let mut log = Log::new();
    let r = Record {
        value: vec![0u8; 42],
        offset: (2),
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
        offset: (2),
    };
    let found = Record {
        value: r.value.clone(),
        offset: (0),
    };

    let _ = log.append(r);
    let res = log.read(0);

    assert!(res.is_some());
    assert_eq!(res.unwrap(), found);
    assert_eq!(log.records.len(), 1);
}
