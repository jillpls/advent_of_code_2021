use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let mut lines = read_lines(&args[0])
        .unwrap()
        .map(|x| x.unwrap())
        .into_iter();
    let mut points = Vec::new();
    let mut fold_instructions = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    while let Some(l) = lines.next() {
        if l.is_empty() {
            break;
        }
        let l = l.split(",").collect::<Vec<&str>>();
        let p = (
            l[0].parse::<usize>().unwrap(),
            l[1].parse::<usize>().unwrap(),
        );
        let p = (p.1, p.0);
        if p.0 > max_x {
            max_x = p.0;
        }
        if p.1 > max_y {
            max_y = p.1;
        }
        points.push(p);
    }
    while let Some(l) = lines.next() {
        if l.is_empty() {
            break;
        }
        let l = l.split("=").collect::<Vec<&str>>();
        fold_instructions.push((l[0].chars().last().unwrap(), l[1].parse::<usize>().unwrap()));
    }

    let mut map = vec![vec![false; max_y + 1]; max_x + 1];

    for p in points {
        map[p.0][p.1] = true;
    }

    println!("({}, {})", map.len(), map[0].len());

    for (dim, line) in fold_instructions {
        println!("{}={}", dim, line);
        match dim {
            'y' => {
                let before_map = &map[0..line];
                let mut after_map = map[line..].iter().collect::<Vec<&Vec<bool>>>();
                after_map.reverse();
                map = before_map
                    .iter()
                    .zip(after_map.iter())
                    .map(|(a, b)| a.iter().zip(b.iter()).map(|(i, j)| *i || *j).collect())
                    .collect::<Vec<Vec<bool>>>();
            }
            'x' => {
                let mut new_map = vec![];
                for row in map {
                    let before_row = &row[0..line];
                    let mut after_row = row[line..].iter().collect::<Vec<&bool>>();
                    after_row.reverse();
                    let row = before_row
                        .iter()
                        .zip(after_row.iter())
                        .map(|(a, b)| *a || **b)
                        .collect::<Vec<bool>>();
                    new_map.push(row);
                }
                map = new_map;
            }
            _ => {}
        }
    }

    let mut count = 0;
    for v in &map {
        for w in v {
            if *w {
                count += 1;
            }
        }
    }

    for v in &map {
        println!(
            "{}",
            v.iter()
                .map(|x| {
                    if *x {
                        '█'
                    } else {
                        ' '
                    }
                })
                .collect::<String>()
        );
    }

    println!("({}, {})", map.len(), map[0].len());
    println!("{}", count);
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
