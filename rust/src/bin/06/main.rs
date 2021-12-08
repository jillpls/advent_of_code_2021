use chrono;
use std::env;
use std::fs;
use std::str::Split;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];

    if args.is_empty() {
        panic!();
    }

    let start = chrono::Utc::now();

    let input = fs::read_to_string(&args[0]).unwrap();

    let fish_input: Split<char> = input.split(',');
    let mut lantern_fish = vec![0; 9];

    for f in fish_input {
        let fish_idx = f.trim().parse::<usize>().unwrap();
        lantern_fish[fish_idx] += 1;
    }

    let mut idx: usize = 0;

    for d in 0..256 {
        let add_idx = (idx + 7) % 9;
        lantern_fish[add_idx] += lantern_fish[idx];
        idx = (idx + 1) % 9;
    }

    let sum: usize = lantern_fish.iter().sum();
    let end = chrono::Utc::now();
    let duration = end - start;

    println!("Total fish: {}", sum);
    println!(
        "Time elapsed: {} ms",
        duration.num_nanoseconds().unwrap_or(0) as f64 / 1000000.0
    );
}
