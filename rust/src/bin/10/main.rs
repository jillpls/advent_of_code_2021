use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let lines: Vec<String> = read_lines(&args[0]).unwrap().map(|x| x.unwrap()).collect();
    let mut corrupted = Vec::new();
    let mut fix_scores = Vec::new();

    'outer: for (i, l) in lines.iter().enumerate() {
        let mut vec: Vec<char> = Vec::new();
        for c in l.chars() {
            let expected = match c {
                '(' | '[' | '{' | '<' => {
                    vec.push(c);
                    continue;
                }
                ')' => '(',
                ']' => '[',
                '}' => '{',
                '>' => '<',
                _ => {
                    break;
                }
            };
            if let Some(d) = vec.pop() {
                if d != expected {
                    corrupted.push((i, c));
                    continue 'outer;
                }
            }
        }
        let mut fix = String::new();
        let mut fix_sum: i64 = 0;
        for c in vec.iter().rev() {
            fix_sum *= 5;
            match c {
                '(' => {
                    fix.push(')');
                    fix_sum += 1;
                }
                '[' => {
                    fix.push(']');
                    fix_sum += 2;
                }
                '{' => {
                    fix.push('}');
                    fix_sum += 3;
                }
                '<' => {
                    fix.push('>');
                    fix_sum += 4;
                }
                _ => {}
            }
        }

        fix_scores.push(fix_sum);
    }
    let mut sum = 0;
    for (_, c) in corrupted {
        match c {
            ')' => sum += 3,
            ']' => sum += 57,
            '}' => sum += 1197,
            '>' => sum += 25137,
            _ => {}
        }
    }

    fix_scores.sort();

    println!("{}", sum);
    println!("{}", fix_scores[fix_scores.len() / 2]);
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
