mod formatter;
mod extractor;

extern crate colored;

use std::collections::HashMap;
use colored::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://hq.sinajs.cn/list=sh601003,sh601001")
        .await?
        .text()
        // .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp.red());
    println!("{}", resp.red().on_yellow());
    println!("{}", resp.red().on_blue());

    let res = reqwest::get("https://hyper.rs").await?;

    println!("Status: {}", res.status().is_success());

    let body = res.text().await?;

    println!("Body:\n\n{}", body);

    println!("{} {} {}", "or use".cyan(), "any".italic().yellow(), "string type".cyan());

    let answer = resp;
    println!("{}", answer.blue());
    Ok(())
}