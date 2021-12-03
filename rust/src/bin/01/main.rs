use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    println!("{:?}", args);
    if args.is_empty() {
        panic!();
    }

    let lines = read_lines(&args[0]).unwrap();

    let numbers = {
        let mut numbers: Vec<u16> = Vec::new();
        for line in lines.flatten() {
            numbers.push(line.parse::<u16>().unwrap());
        }
        numbers
    };

    let mut last_element = numbers[0];
    let mut increases = 0;
    for number in numbers.iter().take(numbers.len() - 1).skip(1) {
        if *number > last_element {
            increases += 1;
        }
        last_element = *number;
    }
    println!("{}", increases);

    let mut window = [numbers[0], numbers[1], numbers[2]];
    increases = 0;
    for number in numbers.iter().skip(3) {
        let old_sum: u16 = window.iter().sum();
        window = [window[1], window[2], *number];
        let new_sum: u16 = window.iter().sum();
        if new_sum > old_sum {
            increases += 1;
        }
    }
    println!("{}", increases);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
