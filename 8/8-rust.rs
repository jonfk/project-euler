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

    //let nums = text_to_ints_functional(s);
    let nums = text_to_ints(s);
    //println!("{:?}", nums);

    // `file` goes out of scope, and the "hello.txt" file gets closed
    let answer = process(nums);
    println!("{}", answer);
}

fn text_to_ints(s: String) -> Vec<u64> {
    let mut result = Vec::new();
    for c in s.chars() {
        if c.is_digit(10) {
            let num = c.to_digit(10).unwrap() as u64;
            result.push(num);
        }
    }
    result
}

fn text_to_ints_functional(s: String) -> Vec<u64> {
    s.chars().filter_map(|c| c.to_digit(10)).map(|x| x as u64).collect::<Vec<_>>()
}

fn process(nums: Vec<u64>) -> u64 {
    let mut i = 0;
    let mut max = 0;
    for n in nums.iter().enumerate() {
        let (i, _) = n;
        let temp = nums.get(i).unwrap_or(&1) * nums.get(i+1).unwrap_or(&1) * nums.get(i+2).unwrap_or(&1) *
            nums.get(i+3).unwrap_or(&1) * nums.get(i+4).unwrap_or(&1) * nums.get(i+5).unwrap_or(&1) * nums.get(i+6).unwrap_or(&1) *
            nums.get(i+7).unwrap_or(&1) * nums.get(i+8).unwrap_or(&1) * nums.get(i+9).unwrap_or(&1) * nums.get(i+10).unwrap_or(&1) *
            nums.get(i+11).unwrap_or(&1) * nums.get(i+12).unwrap_or(&1);

        if temp > max {
            println!("{} {} {} {} {} {} {} {} {} {} {} {} {}",nums.get(i).unwrap_or(&1) , nums.get(i+1).unwrap_or(&1) , nums.get(i+2).unwrap_or(&1), nums.get(i+3).unwrap_or(&1) , nums.get(i+4).unwrap_or(&1) , nums.get(i+5).unwrap_or(&1) , nums.get(i+6).unwrap_or(&1) ,
            nums.get(i+7).unwrap_or(&1) , nums.get(i+8).unwrap_or(&1) , nums.get(i+9).unwrap_or(&1) , nums.get(i+10).unwrap_or(&1) ,
            nums.get(i+11).unwrap_or(&1) , nums.get(i+12).unwrap_or(&1));
            max = temp;
        }
    }

    // while (i+12) < nums.len() {
    //     let temp = nums.get(i).unwrap_or(&1) * nums.get(i+1).unwrap_or(&1) * nums.get(i+2).unwrap_or(&1) *
    //         nums.get(i+3).unwrap_or(&1) * nums.get(i+4).unwrap_or(&1) * nums.get(i+5).unwrap_or(&1) * nums.get(i+6).unwrap_or(&1) *
    //         nums.get(i+7).unwrap_or(&1) * nums.get(i+8).unwrap_or(&1) * nums.get(i+9).unwrap_or(&1) * nums.get(i+10).unwrap_or(&1) *
    //         nums.get(i+11).unwrap_or(&1) * nums.get(i+12).unwrap_or(&1);

    //     if temp > max {
    //         max = temp;
    //     }
    //     i+=1;
    // }
    max
}
