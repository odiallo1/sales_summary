mod lib;

use std::fs;
use std::time::Instant;

fn main() {
    let branches = vec![
        "ALBNM",
        "CTONGA", 
    ];
    
    let output_dir = "data/data/weekly_summary";
    if !fs::metadata(&output_dir).is_ok() {
        fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    }

    let start = Instant::now();

    for branch in &branches {
        match lib::process_input_file(branch) {
            Ok(_) => println!("Processed {}", branch),
            Err(e) => eprintln!("Failed to process {}: {}", branch, e),
        }
    }

    let duration = start.elapsed();
    println!("Processing time: {:?}", duration);

    println!("Phew! I am done.");
}
