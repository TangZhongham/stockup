mod formatter;
mod extractor;

extern crate colored;

use std::collections::HashMap;
use colored::*;
use std::time::Duration;
use std::thread::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let head = format!("{}  {}  {}  {}  {}  {}",
                       "股票名称", "今开", "昨收","当前价格", "今日最高价", "今日最低价");
    println!("{}", head.cyan());
    let five_seconds = Duration::new(5, 0);
    loop {
        run("http://hq.sinajs.cn/list=sz000802").await?;
        sleep(five_seconds);
    }
    Ok(())
}

async fn run(url: &str) -> Result<String, Box<dyn std::error::Error>> {

    let x = extractor::get(url).await?;
    let a = formatter::beautify(x, None);
    // println!("{}", a.unwrap());

    Ok(a.unwrap())
}