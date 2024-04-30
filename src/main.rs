use serde::Deserialize;
use serde_json::Value;
use crate::client::cli::Cli;
use crate::client::http::HttpClient;

mod client;

#[tokio::main]
async fn main() {
    let result = Cli::get("/wx/terminalVeteran/veteranBindTerminals")
        .params(&[("veteranId", "3")])
        .execute::<HttpResult>().await.unwrap();
    println!("{:?}", result);
}

#[derive(Debug,Deserialize)]
struct HttpResult {
    errmsg: String,
    errno: u32,
    data:Value
}