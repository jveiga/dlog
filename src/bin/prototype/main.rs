use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response};

use dlog::{Log, Record};

type State = Arc<Mutex<Log>>;

async fn create_record(mut req: Request<State>) -> tide::Result {
    #[derive(Debug, Deserialize)]
    struct RecordCreateRequest {
        value: String,
    }

    #[derive(Debug, Serialize)]
    struct RecordCreateResponse {
        offset: usize,
    }
    println!("{:?}", "create_record");
    let RecordCreateRequest { value } = req.body_json().await?;
    let state = req.state();
    let mut log = state.lock().unwrap();
    match log.append(Record {
        value: value.into_bytes(),
        offset: 0,
    }) {
        None => unreachable!(),
        Some(offset) => {
            let resp = RecordCreateResponse { offset };
            Ok(Response::builder(201)
                .body(Body::from_json(&resp).unwrap())
                .build())
        }
    }
}

async fn get_record(mut req: Request<State>) -> tide::Result {
    #[derive(Debug, Deserialize)]
    struct RecordGetRequest {
        offset: u32,
    }

    #[derive(Debug, Serialize)]
    struct RecordGetResponse {
        record: RecordResponse,
    }
    #[derive(Debug, Serialize)]
    struct RecordResponse {
        value: Vec<u8>,
        offset: u32,
    }
    println!("{:?}", "get_record");
    let RecordGetRequest { offset } = req.body_json().await?;
    let state = req.state();
    let log = state.lock().unwrap();
    match log.read(offset as usize) {
        None => Ok(Response::builder(404).body("").build()),
        Some(Record {
            value,
            offset: _offset,
        }) => {
            let resp = RecordGetResponse {
                record: RecordResponse { value, offset },
            };
            Ok(Response::builder(200)
                .body(Body::from_json(&resp).unwrap())
                .build())
        }
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let r = State::default();
    let mut app = tide::with_state(r);
    app.at("/").post(create_record).get(get_record);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
