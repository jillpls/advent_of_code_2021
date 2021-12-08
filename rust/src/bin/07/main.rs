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

    let crabs: Vec<i32> = input
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let start_pos = get_median(&crabs);
    let mut direction = 1;
    let mut pos = start_pos;

    let incremental_fuel = true;

    let mut min_fuel = get_fuel(&crabs, pos, incremental_fuel);
    let mut min_pos = pos;
    let fuel = min_fuel;
    println!("{}, {}", pos, fuel);

    loop {
        pos += direction;
        let fuel = get_fuel(&crabs, pos, incremental_fuel);
        println!("{}, {}", pos, fuel);
        if fuel > min_fuel {
            if direction > 0 {
                pos = start_pos;
                direction = -1;
                continue;
            } else {
                break;
            }
        }
        min_pos = pos;
        min_fuel = fuel;
    }
    println!("{}, {}", min_pos, min_fuel);
}

fn get_median(values: &Vec<i32>) -> i32 {
    let mut values = values.clone();
    values.sort();
    if values.len() % 2 == 0 {
        (values[values.len() / 2] + values[values.len() / 2 + 1]) / 2
    } else {
        values[(values.len() + 1) / 2]
    }
}

fn get_fuel(crabs: &Vec<i32>, pos: i32, incremental_fuel: bool) -> i32 {
    if !incremental_fuel {
        crabs.iter().map(|x| (*x - pos).abs()).sum()
    } else {
        crabs
            .iter()
            .map(|x| {
                let d = (*x - pos).abs();
                (d * d + d) / 2
            })
            .sum()
    }
}
