extern crate csv;
extern crate time;
use std::time::{Duration, Instant};

use std::error::Error;
use std::{env, fs};
use time::PreciseTime;

fn main() {
    let now = Instant::now();
    println!("Hello, world!");
    let end = PreciseTime::now();

    let current_dir = "C:\\Users\\SamratKS\\Downloads\\amex-nyse-nasdaq-stock-histories\\fh_20190420\\full_history";

    let entries = fs::read_dir(current_dir).unwrap();

    for entry in entries {
        let path = entry.unwrap().path();
        println!("Meta data : {:#?}", path.file_name().unwrap());
        // println!("Meta data : {:#?}", fs::metadata(entry.unwrap().path()));

        let rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .delimiter(b',')
            .from_path(path)
            .unwrap();
    }
    let new_now = Instant::now();

    println!("{:?}", new_now.duration_since(now));
}

// fn run(current_dir : String) -> Result<(), Error> {
//     for entry in fs::read_dir(current_dir)? {
//         let entry = entry?;
//         let path = entry.path();

//         let metadata = fs::metadata(&path)?;
//         let last_modified = metadata.modified()?.elapsed()?.as_secs();

//         if last_modified < 24 * 3600 && metadata.is_file() {
//             println!(
//                 "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
//                 last_modified,
//                 metadata.permissions().readonly(),
//                 metadata.len(),
//                 path.file_name().ok_or("No filename")?
//             );
//         }
//     }

//     Ok(())
// }
