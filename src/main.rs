mod lib;

use std::fs;
use std::time::Instant;

fn main() {
    let branches = vec![
        "ALBNM",
        "CTONGA",
        "branch1",
        "branch2",
        "branch3",
        "branch4",
        "branch5",
        "branch6",
        "branch7",
        "branch8",
        "branch9",
        "branch10",
        "branch11",
        "branch12",
        "branch13",
        "branch14",
        "branch15",
        "branch16",
        "branch17",
        "branch18",
        "branch19",
        "branch20",
        "branch21",
        "branch22",
        "branch23",
        "branch24",
        "branch25",
        "branch26",
        "branch27",
        "branch28",
        "branch29",
        "branch30",
        "branch31",
        "branch32",
        "branch33",
        "branch34",
        "branch35",
        "branch36",
        "branch37",
        "branch38"
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
