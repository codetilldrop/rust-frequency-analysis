// Basic Frequency Analysis Program

// Importing standard libs to read analysis_text.txt

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut text_file = File::open("analysis-text.txt").expect("file not found");

    let mut file_contents = String::new();
    text_file.read_to_string(&mut file_contents)
        .expect("something went wrong reading the file");
    println!("The file contents is\n{}", file_contents);
}