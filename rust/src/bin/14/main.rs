use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::BufRead;
use std::path::Path;
use std::{env, io};

const MAX_DEPTH: u32 = 40;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let mut lines = read_lines(&args[0])
        .unwrap()
        .map(|x| x.unwrap())
        .into_iter();

    let mut char_count = HashMap::new();
    let mut pair_count = HashMap::new();

    let polymer = lines.next().unwrap();

    for i in 0..polymer.len() - 1 {
        let pair = polymer[i..i + 2].to_string();
        if let Some(v) = pair_count.get_mut(&pair) {
            *v += 1;
        } else {
            pair_count.insert(pair.clone(), 1);
        }
        if let Some(v) = char_count.get_mut(&pair.chars().next().unwrap()) {
            *v += 1;
        } else {
            char_count.insert(pair.chars().next().unwrap(), 1);
        }
    }

    char_count.insert(
        polymer.chars().last().unwrap(),
        char_count
            .get(&polymer.chars().last().unwrap())
            .unwrap_or(&0)
            .clone()
            + 1,
    );

    lines.next();
    let mut map = HashMap::new();
    while let Some(l) = lines.next() {
        let split = l.split("->").collect::<Vec<&str>>();
        map.insert(
            split[0].trim().to_string(),
            split[1].trim().chars().collect::<Vec<char>>()[0],
        );
    }

    for depth in 1..MAX_DEPTH + 1 {
        let mut possible_pairs: Vec<String> = pair_count.keys().map(|x| x.clone()).collect();
        possible_pairs.sort();
        possible_pairs.reverse();
        println!("{}", depth);
        let old_pair_count = pair_count.clone();
        for p in possible_pairs {
            let count = *old_pair_count.get(&p).unwrap();
            if count == 0 {
                continue;
            }

            if !map.contains_key(&p) {
                continue;
            }
            let char = map.get(&p).unwrap();

            add_or_insert(&mut char_count, *char, count);
            let pair_1: String = [p.chars().next().unwrap(), *char].iter().collect();
            let pair_2: String = [*char, p.chars().last().unwrap()].iter().collect();
            add_or_insert(&mut pair_count, pair_1.clone(), count);
            add_or_insert(&mut pair_count, pair_2.clone(), count);
            add_or_insert(&mut pair_count, p.clone(), -count);
        }
    }
    let mut char_count = char_count
        .iter()
        .map(|(k, v)| (*k, *v))
        .collect::<Vec<(char, i64)>>();
    char_count.sort_by(|(_, a), (_, b)| a.cmp(b));

    println!(
        "{:?}",
        char_count.iter().last().unwrap().1 - char_count.iter().next().unwrap().1
    );
}

fn add_or_insert<T: Eq + Hash>(map: &mut HashMap<T, i64>, k: T, v: i64) {
    if let Some(value) = map.get_mut(&k) {
        *value += v;
    } else {
        map.insert(k, v);
    }
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
