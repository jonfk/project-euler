// open.rs
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("./data.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = File::open(&path).unwrap();

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let nums = text_to_ints_functional(s);
    println!("{:?}", nums);

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

fn text_to_ints(s: String) -> Vec<u32> {
    let mut result = Vec::new();
    for c in s.chars() {
        if c.is_digit(10) {
            let num = c.to_digit(10).unwrap();
            result.push(num);
        }
    }
    result
}

fn text_to_ints_functional(s: String) -> Vec<u32> {
    s.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>()
}

fn process(nums: Vec<u32>) -> u32 {
    10
}
