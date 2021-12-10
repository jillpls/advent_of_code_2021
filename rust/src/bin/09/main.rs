use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{env, io};

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let lines: Vec<String> = read_lines(&args[0]).unwrap().map(|x| x.unwrap()).collect();
    let mut map: Vec<Vec<u32>> = Vec::new();
    for _ in lines[0].chars().enumerate() {
        map.push(vec![9; lines.len()])
    }
    for (y, l) in lines.iter().enumerate() {
        for (x, d) in l.chars().enumerate() {
            map[x][y] = d.to_digit(10).unwrap();
        }
    }
    let lowest_values = get_low_points(&map);
    let sum: u32 = lowest_values.iter().sum();
    println!("{}", sum + lowest_values.len() as u32);

    let mut basins = get_basins(&map);
    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    let basins = &basins[0..3];
    let mut product = 1;
    for b in basins {
        product *= b.len();
        println!("{:?}", b.iter().map(|x| x.2).collect::<Vec<u32>>());
    }
    println!("{}", product)
}

fn get_basins(map: &Vec<Vec<u32>>) -> Vec<Vec<(usize, usize, u32)>> {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    let mut pos = (0, 0);
    let mut basins = Vec::new();
    let mut to_visit = vec![(0, 0)];
    let mut current_basin = Vec::new();

    'outer: loop {
        if let Some(p) = to_visit.pop() {
            pos = p;
        } else if let Some(p) = find_next_unvisited(pos, &visited) {
            if !current_basin.is_empty() {
                basins.push(current_basin.clone());
            }
            current_basin.clear();
            pos = p;
        } else {
            if !current_basin.is_empty() {
                basins.push(current_basin);
            }
            break 'outer;
        }

        if visited[pos.0][pos.1] {
            continue;
        }

        let current_val = map[pos.0][pos.1];
        if current_val == 9 {
            visited[pos.0][pos.1] = true;
            continue;
        }

        current_basin.push((pos.0, pos.1, current_val));
        visited[pos.0][pos.1] = true;

        for d in DIRECTIONS {
            if d.0 < -(pos.0 as i32) || d.1 < -(pos.1 as i32) {
                continue;
            }
            let new_pos = ((pos.0 as i32 + d.0) as usize, (pos.1 as i32 + d.1) as usize);
            if new_pos.0 >= map.len() || new_pos.1 >= map[1].len() {
                continue;
            }
            let new_val = map[new_pos.0][new_pos.1];
            if new_val != 9 && !visited[new_pos.0][new_pos.1] {
                to_visit.push(new_pos);
            }
        }
    }
    basins
}

fn get_low_points(map: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut pos = (0, 0);
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    let mut lowest_values = Vec::new();
    'outer: loop {
        if visited[pos.0][pos.1] {
            if let Some(p) = find_next_unvisited(pos, &visited) {
                pos = p;
            } else {
                break 'outer;
            }
        }
        let current_val = map[pos.0][pos.1];
        for d in DIRECTIONS {
            if d.0 < -(pos.0 as i32) || d.1 < -(pos.1 as i32) {
                continue;
            }
            let new_pos = ((pos.0 as i32 + d.0) as usize, (pos.1 as i32 + d.1) as usize);
            if new_pos.0 >= map.len() || new_pos.1 >= map[1].len() {
                continue;
            }
            if map[new_pos.0][new_pos.1] <= current_val {
                visited[pos.0][pos.1] = true;
                pos = new_pos;
                continue 'outer;
            }
        }
        visited[pos.0][pos.1] = true;
        lowest_values.push(map[pos.0][pos.1]);
    }
    lowest_values
}

fn find_next_unvisited(pos: (usize, usize), visited: &Vec<Vec<bool>>) -> Option<(usize, usize)> {
    for i in pos.0..visited.len() - 1 {
        for j in pos.1..visited[0].len() - 1 {
            if !visited[i][j] {
                return Some((i, j));
            }
        }
    }
    for (i, v) in visited.iter().enumerate() {
        for (j, b) in v.iter().enumerate() {
            if !b {
                return Some((i, j));
            }
        }
    }
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
