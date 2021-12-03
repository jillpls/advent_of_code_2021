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
    let codes: Vec<String> = lines.map(|l| l.expect("Error")).collect();

    let code = calculate_code(&codes);

    let gamma_rate = build_code(&code, true).into_iter().collect::<String>();
    let epsilon_rate = build_code(&code, false).into_iter().collect::<String>();

    let gamma_val = val_from_bit_string(gamma_rate.as_ref());
    let epsilon_val = val_from_bit_string(epsilon_rate.as_ref());
    println!(
        "{} * {} = {}",
        gamma_val,
        epsilon_val,
        gamma_val * epsilon_val
    );

    let oxygen_codes = codes.clone();
    let co2_codes = codes;

    let oxygen_code = calculate_reductive_codes(oxygen_codes, true);
    let co2_code = calculate_reductive_codes(co2_codes, false);

    let oxygen_val = val_from_bit_string(oxygen_code.as_ref());
    let co2_val = val_from_bit_string(co2_code.as_ref());
    println!("Oxygen code: {}, Val: {}", oxygen_code, oxygen_val);
    println!("CO2 code: {}, Val: {}", co2_code, co2_val);
    println!("{}", co2_val * oxygen_val);
}

fn calculate_reductive_codes(mut codes: Vec<String>, most_common: bool) -> String {
    let mut idx = 0;
    while codes.len() > 1 {
        let code = calculate_code(&codes);
        let gamma_rate = build_code(&code, most_common);
        let keep_bit = gamma_rate[idx];
        codes.retain(|x| x.chars().collect::<Vec<char>>()[idx] == keep_bit);
        idx += 1;
    }
    codes[0].clone()
}

fn calculate_code(codes: &[String]) -> Vec<i32> {
    let len = codes.get(0).unwrap().chars().count();

    let mut final_code: Vec<i32> = vec![0; len];

    for code in codes {
        for (idx, digit) in code.chars().collect::<Vec<char>>().iter().enumerate() {
            if digit == &'0' {
                final_code[idx] -= 1;
            } else {
                final_code[idx] += 1;
            }
        }
    }

    final_code
}

fn build_code(code: &[i32], most_common: bool) -> Vec<char> {
    code.iter()
        .map(|x| {
            let x = if most_common { *x } else { -1 * x };
            if x > 0 {
                '1'
            } else if x < 0 {
                '0'
            } else if most_common {
                '1'
            } else {
                '0'
            }
        })
        .collect::<Vec<char>>()
}

fn val_from_bit_string(bit_string: &str) -> i32 {
    i32::from_str_radix(bit_string, 2).unwrap()
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
