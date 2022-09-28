// This example requires the following input to succeed:
// { "command": "do something" }

use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};


use rustddd::dominio01::*;
use std::collections::HashMap;

struct DummyPersistable {
    dummy_database: HashMap<u32, String>,
}
impl DummyPersistable {
    fn init(&mut self) {
        self.dummy_database = HashMap::new();
        self.dummy_database.insert(1, String::from("hola"));
        self.dummy_database.insert(2, String::from("adios"));
        self.dummy_database.insert(3, String::from("mundo"));
        self.dummy_database.insert(4, String::from("nube"));
    }
}
impl Persistable for DummyPersistable {
    fn save(&self, id: u32, message: String) {
        println!("{:?} {:?}", id, message);
    }

    fn load(&self, id: u32) -> String {
        println!("{:?} ", id);
        let result: String = match self.dummy_database.get(&id) {
            Some(m) => m.to_string(),
            None => String::from("NAN"),
        };
        result
    }
}

/// This is also a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    command: String,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
    dummy_msg: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let func = service_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // extract some useful info from the request
    let command = event.payload.command;
    
        let mut m01 = Dominio01 {
        id: 1,
        message: String::from(""),
    };
    let mut dummy_repo = DummyPersistable {
        dummy_database: HashMap::new(),
    };
    dummy_repo.init();
    let msg: String = m01.logic(&dummy_repo);

    // prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("Command {} executed.", command),
        dummy_msg: format!("DummyMessage {} .", msg),
    };

    // return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}
