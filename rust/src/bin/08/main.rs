use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];

    let numbers: HashMap<u8, HashSet<char>> = HashMap::from([
        (0, "abcefg".chars().collect()),
        (1, "cf".chars().collect()),
        (2, "acdeg".chars().collect()),
        (3, "acdfg".chars().collect()),
        (4, "bcdf".chars().collect()),
        (5, "abdfg".chars().collect()),
        (6, "abdefg".chars().collect()),
        (7, "acf".chars().collect()),
        (8, "abcdefg".chars().collect()),
        (9, "abcdfg".chars().collect()),
    ]);

    let mut numbers_by_count: HashMap<usize, Vec<u8>> = HashMap::new();

    for (k, v) in &numbers {
        if let Some(c) = numbers_by_count.get_mut(&v.len()) {
            c.push(*k);
        } else {
            numbers_by_count.insert(v.len(), vec![*k]);
        }
    }

    if args.is_empty() {
        panic!();
    }

    let lines = read_lines(&args[0]).unwrap();

    let mut values = Vec::new();

    for l in lines {
        let l = l.unwrap();
        let split: Vec<String> = l.split("|").map(|x| x.to_string()).collect();
        let output_val: Vec<String> = split[1]
            .trim()
            .split(" ")
            .map(|x| x.trim().to_string())
            .collect();
        let input_val: Vec<String> = split[0]
            .trim()
            .split(" ")
            .map(|x| x.trim().to_string())
            .collect();
        values.push((input_val, output_val));
    }

    let mut count = 0;

    for o in &values {
        let o = &o.1;
        for s in o {
            let l = s.len();
            if l == 2 || l == 3 || l == 4 || l == 7 {
                count += 1;
            }
        }
    }

    println!("{}", count);
    let mut sum = 0;

    for (v, o) in &values {
        let mut v = v.iter().collect::<HashSet<&String>>();
        let mut lookup: HashMap<u8, HashSet<char>> = HashMap::new();
        loop {
            let mut decoded: Vec<&String> = Vec::new();
            for i in &v {
                let str_set = i.chars().collect::<HashSet<char>>();
                match i.len() {
                    2 => {
                        lookup.insert(1, i.chars().collect());
                        decoded.push(i);
                    } // 1
                    3 => {
                        lookup.insert(7, i.chars().collect());
                        decoded.push(i);
                    } // 7
                    4 => {
                        lookup.insert(4, i.chars().collect());
                        decoded.push(i);
                    } // 4
                    7 => {
                        lookup.insert(8, i.chars().collect());
                        decoded.push(i);
                    } // 8
                    5 => {
                        // let mut possible = HashSet::from([2, 3, 5]);
                        if lookup.contains_key(&3) {
                            if let Some(set) = lookup.get(&4) {
                                if set.difference(&str_set).collect::<HashSet<&char>>().len() == 1 {
                                    lookup.insert(5, str_set);
                                    decoded.push(i);
                                    continue;
                                } else {
                                    lookup.insert(2, str_set);
                                    decoded.push(i);
                                    continue;
                                }
                            }
                        }
                        if let Some(set) = lookup.get(&1) {
                            if set
                                .difference(&str_set)
                                .collect::<HashSet<&char>>()
                                .is_empty()
                            {
                                lookup.insert(3, str_set);
                                decoded.push(i);
                                continue;
                            }
                        }
                    } // 2, 3, 5
                    6 => {
                        if lookup.contains_key(&9) {
                            if let Some(set) = lookup.get(&1) {
                                if set
                                    .difference(&str_set)
                                    .collect::<HashSet<&char>>()
                                    .is_empty()
                                {
                                    lookup.insert(0, str_set);
                                    decoded.push(i);
                                    continue;
                                } else {
                                    lookup.insert(6, str_set);
                                    decoded.push(i);
                                    continue;
                                }
                            }
                        }
                        if let Some(set) = lookup.get(&4) {
                            if set
                                .difference(&str_set)
                                .collect::<HashSet<&char>>()
                                .is_empty()
                            {
                                lookup.insert(9, str_set);
                                decoded.push(i);
                                continue;
                            }
                        }
                    } // 0, 6, 9
                    _ => {}
                }
            }
            for d in decoded {
                v.remove(d);
            }
            if v.is_empty() {
                break;
            }
        }

        let mut inverse_lookup = HashMap::new();

        for (key, value) in lookup {
            let mut vec: Vec<char> = value.iter().cloned().collect();
            vec.sort();
            inverse_lookup.insert(vec, key);
        }

        let mut factor = 1000;
        let mut number = 0;
        for n in o {
            let mut vec: Vec<char> = n.chars().collect();
            vec.sort();
            let digit = inverse_lookup.get(&vec).unwrap();
            number += (*digit as u32) * factor;
            factor /= 10;
        }
        sum += number;
    }
    println!("{}", sum);
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
