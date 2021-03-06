mod storage;
mod reader;
mod formatter;
mod extractor;

extern crate colored;

use std::collections::HashMap;
use reqwest::IntoUrl;
use colored::*;
use std::time::Duration;
use std::thread::sleep;

use clap::{Arg, App};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let matches = App::new("Stock it up...")
        .version("v0.5")
        .author("Tang Zhongham <13122260573@163.com>")
        .about("监控股票价格: ./stockup sz000802,sz000803\n\
               ./stockup -h")
        .arg(Arg::with_name("stock_list")
            .short("s".parse().unwrap()).long("stocks")
            .takes_value(true).help("股票列表格式: sz000802,sz000803"))
        .arg(Arg::with_name("refresh_time")
            .short("t".parse().unwrap()).long("time")
            .takes_value(true).help("刷新时间, 默认5s"))
        .get_matches();

    let default_stock = "sz000803,sz000801";
    let source_url = "http://hq.sinajs.cn/list=".to_string();
    let _stock_list = matches.value_of("stock_list").unwrap_or(default_stock);
    let stock_list: Vec<&str> = _stock_list.split(",").collect();

    // 默认刷新时间
    let refresh_time:u64 = matches.value_of_t("refresh_time").unwrap_or(5);

    let head = format!("{}  {}  {}  {}  {}  {}",
                       "股票名称", "今开", "昨收","当前价格", "今日最高价", "今日最低价");
    println!("{}", &head.cyan());

    let five_seconds = Duration::new(refresh_time, 0);

    loop {

        for &stock in &stock_list {
            let url = format!("{}{}", &source_url, &stock);
            let mut v = vec![1.0];
            let former_price = v.pop();
            let price = run(&url, former_price.unwrap_or(1.0)).await?;
            v.push(price);
        }
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


    // let a = formatter::beautify(x, Some(greater));
    let a = match formatter::beautify(x, Some(greater)) {
        Ok(a) => a,
        Err(error) => {
            panic!("Problem beatifying results: {:?}", error)
        }
    };

    Ok(price)
}

#[tokio::test]
async fn test_url() -> Result<(), Box<dyn std::error::Error>> {
    let default_stock = "sz000802,yyy";
    let matches = App::new("Stock on my way...")
        .version("0.5")
        .author("Tang Zhongham <13122260573@163.com")
        .about("监控股票价格: sz000802")
        .arg(Arg::with_name("stock_list")
            .short("s".parse().unwrap()).long("stocks")
            .takes_value(true).help("股票列表: sz000802,sz000803"))
        .get_matches();
    let stock_list = matches.value_of("stock_list").unwrap_or(default_stock);

    let x = stock_list.split(",");
    println!("{}", x.count());
    // let source_url = "http://hq.sinajs.cn/list=".to_string();
    // let url = source_url + stock_list;
    // println!("{}", url);
    Ok(())
}