mod lib;

use std::fs;
use std::time::Instant;

fn main() {
    let output_dir = "sales_summary/data/weekly_summary";
    if !fs::metadata(&output_dir).is_ok() {
        fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    }

    let start = Instant::now();

    for i in 1..=40 {
        match lib::process_input_file(i) {
            Ok(_) => println!("Processed branch{}", i),
            Err(e) => eprintln!("Failed to process branch{}: {}", i, e),
        }
    }

    let duration = start.elapsed();
    println!("Processing time: {:?}", duration);

    println!("Phew! I am done.");
}
