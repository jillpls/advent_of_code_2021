use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    println!("{:?}", args);
    if args.len() <= 0 {
        panic!();
    }

    let numbers = if let Ok(lines) =  read_lines(&args[0]) {
        let mut numbers : Vec<u16> = Vec::new();
        for line in lines {
            if let Ok(l) = line {
                numbers.push(l.parse::<u16>().unwrap());
            }
        }
        numbers
    } else {
        Vec::new()
    };

    let mut last_element = numbers[0];
    let mut increases = 0;
    for i in 1..numbers.len()-1 {
        if numbers[i] > last_element {
            increases += 1;
        }
        last_element = numbers[i];
    }
    println!("{}", increases);

    let mut window = [numbers[0], numbers[1], numbers[2]];
    increases = 0;
    for i in 3..numbers.len() {
        let old_sum : u16 = window.iter().sum();
        window = [window[1], window[2], numbers[i]];
        let new_sum : u16 = window.iter().sum();
        if new_sum > old_sum {
            increases += 1;
        }

    }
    println!("{}", increases);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}