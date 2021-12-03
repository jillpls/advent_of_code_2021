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

    let instructions = {
        let mut instructions: Vec<(String, i32)> = Vec::new();
        for line in lines.flatten() {
            let l_split: Vec<&str> = line.split(' ').collect();
            let instruction = (
                String::from(l_split[0]),
                String::from(l_split[1]).parse::<i32>().unwrap(),
            );
            instructions.push(instruction);
        }
        instructions
    };

    let mut pos: (i32, i32) = (0, 0);

    for instr in &instructions {
        match instr.0.as_ref() {
            "forward" => {
                pos.0 += instr.1;
            }
            "up" => {
                pos.1 -= instr.1;
            }
            "down" => {
                pos.1 += instr.1;
            }
            _ => {}
        }
    }

    println!("{} * {} = {}", pos.0, pos.1, pos.0 * pos.1);

    let mut aim: i32 = 0;
    pos = (0, 0);

    for instr in &instructions {
        match instr.0.as_ref() {
            "forward" => {
                pos.0 += instr.1;
                pos.1 += instr.1 * aim;
            }
            "up" => {
                aim -= instr.1;
            }
            "down" => {
                aim += instr.1;
            }
            _ => {}
        }
    }

    println!("{} * {} = {}", pos.0, pos.1, pos.0 * pos.1);
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
