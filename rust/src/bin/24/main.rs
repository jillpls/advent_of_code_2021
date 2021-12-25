use std::collections::{HashMap};
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufRead};
use std::iter::Iterator;
use std::path::Path;
use std::slice::Iter;

#[derive(Debug)]
enum Instruction {
    Input(Number),
    Add(Number, Number),
    Mul(Number, Number),
    Div(Number, Number),
    Mod(Number, Number),
    Eql(Number, Number),
}

impl Instruction {
    fn apply(&self, values: &mut [i64; 4], input: i64) {
        match self {
            Self::Input(a) => {
                values[a.get_idx().unwrap()] = input;
            }
            Self::Add(a, b) => {
                values[a.get_idx().unwrap()] = a.unwrap(&values) + b.unwrap(&values);
            }
            Self::Mul(a, b) => {
                values[a.get_idx().unwrap()] = a.unwrap(&values) * b.unwrap(&values);
            }
            Self::Div(a, b) => {
                values[a.get_idx().unwrap()] = a.unwrap(&values) / b.unwrap(&values);
            }
            Self::Mod(a, b) => {
                values[a.get_idx().unwrap()] = a.unwrap(&values) % b.unwrap(&values);
            }
            Self::Eql(a, b) => {
                values[a.get_idx().unwrap()] = if a.unwrap(&values) == b.unwrap(&values) {
                    1
                } else {
                    0
                };
            }
        }
    }

    #[allow(dead_code)]
    fn apply_iter(&self, values: &mut [i64; 4], input: &mut Iter<i64>) {
        self.apply(values, input.next().copied().unwrap())
    }
}

#[derive(Copy, Clone, Debug)]
enum Number {
    Index(usize),
    Literal(i64),
}

impl Number {
    fn unwrap(&self, values: &[i64; 4]) -> i64 {
        match self {
            Self::Index(idx) => values[*idx],
            Self::Literal(v) => *v,
        }
    }

    fn get_idx(&self) -> Option<usize> {
        if let Self::Index(idx) = self {
            Some(*idx)
        } else {
            None
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let lines = read_lines(&args[0])
        .unwrap()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();
    let mut instructions = Vec::new();

    for line in lines {
        let split = line.split(" ").collect::<Vec<&str>>();
        let values = [split[1], split.get(2).unwrap_or(&"0")];
        let mut processed_values = [Number::Index(0), Number::Index(0)];
        for i in 0..values.len() {
            processed_values[i] = match values[i] {
                "w" => Number::Index(0),
                "x" => Number::Index(1),
                "y" => Number::Index(2),
                "z" => Number::Index(3),
                _ => Number::Literal(values[i].parse::<i64>().unwrap_or(0)),
            };
        }

        let instruction: Option<Instruction> = match split[0] {
            "inp" => Some(Instruction::Input(processed_values[0])),
            "add" => Some(Instruction::Add(processed_values[0], processed_values[1])),
            "mul" => Some(Instruction::Mul(processed_values[0], processed_values[1])),
            "div" => Some(Instruction::Div(processed_values[0], processed_values[1])),
            "mod" => Some(Instruction::Mod(processed_values[0], processed_values[1])),
            "eql" => Some(Instruction::Eql(processed_values[0], processed_values[1])),
            _ => panic!(),
        };
        if let Some(i) = instruction {
            instructions.push(i);
        }
    }

    let r = find_best(&instructions, 0, [0; 4], true, &mut HashMap::new());
    println!("{}", r.unwrap());
    let r = find_best(&instructions, 0, [0; 4], false, &mut HashMap::new());
    println!("{}", r.unwrap());
}

fn find_best(
    instructions: &[Instruction],
    idx: usize,
    alu_values: [i64; 4],
    high: bool,
    cache: &mut HashMap<([i64; 4], usize), Option<i64>>,
) -> Option<i64> {
    if let Some(r) = cache.get(&(alu_values, idx)) {
        return *r;
    }

    let input = if high {
        [9, 8, 7, 6, 5, 4, 3, 2, 1]
    } else {
        [1, 2, 3, 4, 5, 6, 7, 8, 9]
    };
    'outer: for i in input {
        let mut alu_values = alu_values;
        let mut idx = idx;
        let input_instr = instructions.get(idx).unwrap();
        if let Instruction::Input(_) = input_instr {
            input_instr.apply(&mut alu_values, i);
            idx += 1;
        } else {
            panic!();
        }
        while let Some(instruction) = instructions.get(idx) {
            if let Instruction::Input(_) = instruction {
                let res = find_best(instructions, idx, alu_values, high, cache);
                if let Some(r) = res {
                    let r = r / 10 + i * 10i64.pow(13);
                    cache.insert((alu_values, idx), Some(r));
                    return Some(r);
                } else {
                    continue 'outer;
                }
            } else {
                instruction.apply(&mut alu_values, 0);
                idx += 1;
            }
        }

        if alu_values[3] == 0 {
            cache.insert((alu_values, idx), Some(i));
            return Some(i * 10i64.pow(13));
        }
    }

    cache.insert((alu_values, idx), None);
    None
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
