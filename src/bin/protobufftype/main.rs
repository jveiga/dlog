// use tracing::Level;
// use tracing_log::LogTracer;
// use tracing_subscriber::FmtSubscriber;

// use dlog::{Config, Segment};

fn main() -> std::io::Result<()> {
    // LogTracer::init().unwrap();
    // setup_tracing();
    // let mut s = Segment::new("/home/jveiga/rust/dlog", 0, Config::default())?;
    // let (value, offset) = ("abcd".into(), 2);
    // let r = dlog::Record { value, offset };
    // println!("append r {:?}", s.append(r).unwrap());
    // let r2 = dlog::Record {
    //     value: "efgh".into(),
    //     offset: 2,
    // };
    // println!("read {:?}", s.read(0));
    // println!("append r2 {:?}", s.append(r2).unwrap());
    // println!("{:?}", s.read(1));
    // s.append("abcd".as_bytes()).unwrap();
    // s.append("efgh".as_bytes()).unwrap();

    // s.append("ijkl".as_bytes()).unwrap();

    // println!("{:?}", String::from_utf8(s.read(0)?).unwrap());
    // println!("{:?}", String::from_utf8(s.read(12)?).unwrap());
    // let mut buf = [0u8; 4];
    // s.read_at(&mut buf, 8)?;

    // idx.write(0, 10).unwrap();
    // println!("{:?}", idx.read(0).unwrap());

    Ok(())
}

// fn setup_tracing() {
//     let subscriber = FmtSubscriber::builder()
//         // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
//         // will be written to stdout.
//         .with_max_level(Level::TRACE)
//         // completes the builder.
//         .finish();

//     tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
// }
