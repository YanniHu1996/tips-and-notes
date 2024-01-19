use reqwest::header;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListItem {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    list: Vec<ListItem>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();

    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/json, text/plain, */*"),
    );
    headers.insert(
        header::ACCEPT_LANGUAGE,
        header::HeaderValue::from_static("zh-CN,zh;q=0.9,en;q=0.8"),
    );
    headers.insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("no-cache"),
    );

    env::var("XUEQIU_COOKIE").map(|cookie| {
        headers.insert(
            header::COOKIE,
            header::HeaderValue::from_str(&cookie).unwrap(),
        );
    })?;

    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"),
    );

    let cli = reqwest::blocking::Client::new();
    let resp = cli
        .get("https://xueqiu.com/query/v1/symbol/search/status.json")
        .headers(headers)
        .query(&[("count", "10")])
        .query(&[("comment", "0")])
        .query(&[("symbol", "SH000905")])
        .query(&[("hl", "0")])
        .query(&[("source", "user")])
        .query(&[("sort", "time")])
        .query(&[("page", "1")])
        .query(&[("q", "")])
        .query(&[("type", "12")])
        .send()?;

    let body = resp.json::<Root>()?;

    body.list.iter().for_each(|item| {
        println!("{}\n", item.text);
    });

    Ok(())
}
