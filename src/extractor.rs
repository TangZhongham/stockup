// use std::collections::HashMap;
// use std::io::{BufRead, Read};
// use std::fmt::Debug;

use std::collections::HashMap;
use std::process::exit;

// pub struct Stock {
//     // 股票id
//     id: String
// }

// pub struct Stock {
//     // 股票id
//     id: String,
//     // 股票名称
//     name: String,
//     // 今日开盘价
//     open: i16,
//     // 昨日收盘价
//     close_yesterday: i16,
//     // 当前价格
//     price: i16,
//     // 今日最高价
//     high: i16,
//     // 今日最低价
//     low: i16,
//     // 时间
//     time: String
// }

// #[tokio::main]
pub async fn get(url: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {

    // 针对每个股票异步执行
    // let client = reqwest::Client::new();

    // 想来还是 hashmap 比较符合，struct 活动性比较差
    // let mut results = HashMap::new();


    let resp = reqwest::get(url)
        .await?;

    if !resp.status().is_success() {
        exit(1);
    }

    // println!("{}", resp.status());

    let mut mm = resp.text().await?;

    let v: Vec<&str> = mm.split(",").collect();
    let mut results = HashMap::new();
    let _name: Vec<&str> = v[0].split("=\"").collect();
    let name = _name[1];
    results.insert("股票名称".to_string(), name.to_string());
    results.insert("今开".to_string(), v[1].to_string());
    results.insert("昨收".to_string(), v[2].to_string());
    results.insert("当前价格".to_string(), v[3].to_string());
    results.insert("今日最高价".to_string(), v[4].to_string());
    results.insert("今日最低价".to_string(), v[5].to_string());

    // Ok(Stock{id:"s".to_string()})
    Ok(results)
}


#[tokio::test]
async fn my_test() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://hq.sinajs.cn/list=sh601003";
    // let target = "";
    let results = get(url).await?;
    for (name, value) in &results {
        println!("fuck {}: \"{}\"", name, value);
    }
    // assert!(true);

    let mm = "var hq_str_sh601003=\"柳钢股份,5.280,5.280,5.320,5.380,5.220,5.310,5.320,14702002,77860257.000,21700,5.310,89800,5.300,11500,5.290,51100,5.280,88800,5.270,107640,5.320,69100,5.330,203900,5.340,447754,5.350,241356,5.360,2020-07-22,11:05:51,00,\";
";
    // for c in mm.chars() {
    //     println!("{}", c);
    // }
    let s = &mm[0..21];
    println!("{}", s);
    let mut book_reviews = HashMap::new();

    let v: Vec<&str> = mm.split(",").collect();
    let _name: Vec<&str> = v[0].split("=\"").collect();
    let name = _name[1];
    println!("NAME is {}", name);
    book_reviews.insert("股票名称".to_string(), name.to_string());
    book_reviews.insert("今日开盘价".to_string(), v[1].to_string());
    book_reviews.insert("昨日收盘价".to_string(), v[2].to_string());
    book_reviews.insert("当前价格".to_string(), v[3].to_string());
    book_reviews.insert("今日最高价".to_string(), v[4].to_string());
    book_reviews.insert("今日最低价".to_string(), v[5].to_string());

    for (book, review) in &book_reviews {
        println!("fuck {}: \"{}\"", book, review);
    }
    println!("xxx {}", v[1]);

    for i in v {
        println!("{}", i);
    }

    Ok(())
}