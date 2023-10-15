use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};

pub fn process_input_file(branch_number: i32) -> Result<String, io::Error> {
    let input_file_path = format!("sales_summary/data/branch{}/branch_weekly_sales.txt", branch_number);
    let file = match std::fs::File::open(&input_file_path) {
        Ok(file) => file,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("Failed to open file: {}", e))),
    };
    let reader = BufReader::new(file);

    let mut total_sales = 0;
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("Failed to read line: {}", e))),
        };
        let parts: Vec<&str> = line.split(", ").collect();
        if let Ok(sales) = parts[2].parse::<i32>() {
            total_sales += sales;
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to parse sales data"));
        }
    }
    
    let output_str = format!("branch{}, PROD001, {}", branch_number, total_sales);
    write_to_summary_file(&output_str)?;
    Ok("OK".to_string())
}

pub fn write_to_summary_file(output_str: &str) -> Result<(), io::Error> {
    let output_file_path = "sales_summary/data/weekly_summary/weekly_sales_summary.txt";
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(output_file_path)?;
    writeln!(file, "{}", output_str)?;
    Ok(())
}
