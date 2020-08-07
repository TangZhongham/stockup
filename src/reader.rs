// // use csv::Error;
// use csv::Reader;
// use std::io;
// use std::path::Path;
// use std::fs::File;
//
// use std::io::{Write, BufReader, BufRead};
// use std::error::Error;
//
// pub fn read_list(dir: &Path) {
//
//     let mut rdr = Reader::from_path(dir)?;
//         for result in rdr.records() {
//             let record = result?;
//             println!("{:?}", record);
//         }
// }
//
// #[test]
// fn test_read_list()  {
//     let mut path = Path::new("stock_listt.csv");
//     let mut file = File::create(path)?;
//
//     write!(file, "FUCK")?;
//
//     let input = File::open(path)?;
//     let buffered = BufReader::new(input);
//
//     for line in buffered.lines() {
//         println!("{}", line?);
//     }
//
//     let f = File::open("foo.txt")?;
//     let mut reader = BufReader::new(f);
//     let mut buffer = String::new();
//
//     // read a line into buffer
//     reader.read_line(&mut buffer)?;
//
//     println!("{}", buffer);
//
//     // let mut rdr = Reader::from_path(path)?;
//     // for result in rdr.records() {
//     //     let record = result?;
//     //     println!("{:?}", record);
//     // }
//
//     let path = Path::new("hello.txt");
//     let display = path.display();
//
//     // Open the path in read-only mode, returns `io::Result<File>`
//     let mut file = match File::open(&path) {
//         // The `description` method of `io::Error` returns a string that
//         // describes the error
//         Err(why) => panic!("couldn't open {}: {}", display,
//                            why.description()),
//         Ok(file) => file,
//     };
//
//
// }