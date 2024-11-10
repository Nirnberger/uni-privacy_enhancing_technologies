use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
// Function to parse a string into an integer, returns Result type
pub fn parse_integer(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("Failed to parse {} as an integer", s)),
    }
}

// File I/O

// Function to read all lines from a file, returns Result type with Vec<String>
pub fn read_file_lines<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}

// Function to write a string to a file, returns Result type
pub fn write_file<P: AsRef<Path>>(file_path: P, content: &str) -> Result<(), io::Error> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

// Combining Error Handling and File I/O

// Function to read integers from file, compute their sum, and return the result
pub fn read_and_sum_integers<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let lines = read_file_lines(&file_path).map_err(|x| format!("Failed to read file: {}",x))?;

    let sum: i32 = lines
        .iter()
        .map(|line| line.parse::<i32>().map_err(|e| format!("Failed to parse '{}': {}", line, e)))
        .collect::<Result<Vec<_>, _>>()? // Collect parsed integers into a Result<Vec<i32>, String>
        .into_iter()
        .sum();

    let result_file_path = file_path.as_ref().with_extension("result");
    write_file(result_file_path, &sum.to_string()).map_err(|e| format!("Failed to write file: {}", e))?;
    Ok(sum)
}
