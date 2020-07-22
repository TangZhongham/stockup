mod formatter;
mod extractor;

extern crate colored;

use std::collections::HashMap;
use colored::*;
use std::time::Duration;
use std::thread::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let v = vec![];
    // let mut v: Vec<f32> = Vec::new();
    let mut v = vec![1.0];

    let head = format!("{}  {}  {}  {}  {}  {}",
                       "股票名称", "今开", "昨收","当前价格", "今日最高价", "今日最低价");
    println!("{}", head.cyan());
    let five_seconds = Duration::new(5, 0);
    loop {
        let former_price = v.pop();
        println!("{}", former_price.unwrap().to_string());
        let price = run("http://hq.sinajs.cn/list=sz000802", former_price.unwrap_or(1.0)).await?;
        v.push(price);
        sleep(five_seconds);
    }
    Ok(())
}

async fn run(url: &str, former_price: f32) -> Result<f32, Box<dyn std::error::Error>> {

    let x = extractor::get(url).await?;
    let price = x["当前价格"].parse().unwrap();
    // 判断是否大于等于之前的价格，显示红色/绿色
    let greater = if price >= former_price {
        true
    } else {
        false
    };

    println!("{}", greater.to_string());

    let a = formatter::beautify(x, Some(greater));
    // println!("{}", a.unwrap());

    Ok(price)
}