extern crate colored;
use colored::*;

pub fn beautify(line: &str) {
    println!("{}", line.blue());
    let v = vec!["a", "b", "c", "d"];

    for i in v.iter() {
        println!("A reference to {}", i);
    };

    let r = String::from("xyz").green().bold();
    // r
}

#[test]
fn test_beautify() {
    let a = beautify("a");
}