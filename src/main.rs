use reqwest::Client;
use serde::Deserialize;

mod lib;

use crate::lib::get_key_from_json;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let url = "https://zipcloud.ibsnet.co.jp/api/search";
    let response = client
        .get(url)
        .query(&[("zipcode", "1000002")])
        .send()
        .await?;
    let body = response.text().await?;
    let keys: Vec<String> = get_key_from_json(body);
    for key in keys {
        println!("{}", key);
    }
    Ok(())
}
