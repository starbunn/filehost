extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use warp::Filter;

#[derive(Deserialize, Serialize, PartialEq)]
enum RequestType {
    Content,
    DateCreated,
    LastUpdated,
}

#[derive(Deserialize, Serialize)]
struct Query {
    path: String,
    info: RequestType,
}

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    // let hello = warp::path!("hello" / String)
    // .map(|name| format!("Hello, {}!", name));

    // let file = warp::path!("file" / String).map(|filename| {
    //     let response = fs::read_to_string("src/main.rs").expect("file not found");
    //     response
    // });

    let file = warp::get()
        .and(warp::path("file"))
        .and(warp::query::<Query>())
        .map(|res: Query| match res.info {
            RequestType::Content => {
                let path = res.path.replace("..", "");
                let response = fs::read_to_string(path);
                if response.is_ok() {
                    response.unwrap()
                } else {
                    "file not found".to_string()
                }
            }
            RequestType::DateCreated => {
                let path = res.path.replace("..", "");
                let response = fs::metadata(path);
                if response.is_ok() {
                    let datetime: DateTime<Utc> = response.unwrap().created().unwrap().into();
                    format!("{}", datetime.format("%d/%m/%Y %T"))
                } else {
                    "NONE".to_string()
                }
            }
            RequestType::LastUpdated => {
                let path = res.path.replace("..", "");
                let response = fs::metadata(path);
                if response.is_ok() {
                    let datetime: DateTime<Utc> = response.unwrap().modified().unwrap().into();
                    format!("{}", datetime.format("%d/%m/%Y %T"))
                } else {
                    "NONE".to_string()
                }
            }
        });

    // let home = warp::path::end().map(|| "Hello, world!");

    let routes = file;

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
