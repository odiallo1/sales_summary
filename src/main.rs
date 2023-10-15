mod lib;

use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} /sales_summary/data", args[0]);
        process::exit(1);
    }
    let data_folder_path = &args[1];

    let output_dir = format!("{}/weekly_summary", data_folder_path);
    if !fs::metadata(&output_dir).is_ok() {
        fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    }

    let start = std::time::Instant::now();

    for i in 1..=40 {
        let folder_path = format!("{}/branch{}", data_folder_path, i);
        match lib::process_input_file(&folder_path) {
            Ok(_) => println!("Processed branch{}", i),
            Err(e) => eprintln!("Failed to process branch{}: {}", i, e),
        }
    }

    let duration = start.elapsed();
    println!("Processing time: {:?}", duration);

    println!("Phew! I am done.");
}
