extern crate colored;
use colored::*;
use std::collections::HashMap;
use super::extractor::get;

pub fn beautify(line: HashMap<String, String>, former_price: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    // TODO: 添加字段 high/low 变化当前价格
    // let former_price: i32 = former_price.parse().unwrap_or(0);
    //
    // println!("{}", former_price.to_string());

    // for val in line.values() {
    //     println!("{}", val.blue());
    // }
    // let v = vec!["a", "b", "c", "d"];

    // for i in v.iter() {
    //     println!("A reference to {}", i);
    // };
    let hello_world = format!("{}  {}  {}  {}  {}  {}", line["股票名称"], line["今开"], line["昨收"], line["当前价格"].to_string().red(), line["今日最高价"], line["今日最低价"]);

    let r = String::from(hello_world).black().bold().on_cyan();
    println!("{}", r);
    // r
    // Ok(r.to_string())
    Ok(line["当前价格"].to_string())
}

#[tokio::test]
async fn test_beautify() -> Result<(), Box<dyn std::error::Error>> {
    let x = get("http://hq.sinajs.cn/list=sh601003").await?;
    let a = beautify(x, None);
    println!("{}", a.unwrap());
    let world = "world".bold();
    let hello_world = format!("Hello, {}!", world);
    println!("{}", hello_world);
    let hello_world = format!("Hello, {}!lalalala", world).red();
    println!("{}", hello_world);
    Ok(())
}