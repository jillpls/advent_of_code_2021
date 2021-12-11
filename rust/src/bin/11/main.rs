use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{env, io};

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let lines: Vec<String> = read_lines(&args[0]).unwrap().map(|x| x.unwrap()).collect();
    let mut map = vec![vec![0; lines[0].len()]; lines.len()];
    for (x, l) in lines.iter().enumerate() {
        for (y, c) in l.chars().enumerate() {
            map[x][y] = c.to_digit(10).unwrap();
        }
    }
    let steps = 0;
    let max_flashes = map.len() * map[0].len();
    let mut flashes = Vec::new();
    let mut flashes_count = 0;
    let mut i = 0;
    loop {
        let mut step_flashes_count = 0;
        for x in 0..map.len() {
            for y in 0..map[0].len() {
                let d = map[x][y];
                if d >= 9 {
                    flashes.push((x, y));
                    map[x][y] = 0;
                } else {
                    map[x][y] += 1;
                }
            }
        }
        while let Some((x, y)) = flashes.pop() {
            step_flashes_count += 1;
            for dir in DIRECTIONS {
                let new_x = x as i32 + dir.0;
                let new_y = y as i32 + dir.1;
                if new_x < 0
                    || new_y < 0
                    || new_x >= map.len() as i32
                    || new_y >= map[0].len() as i32
                {
                    continue;
                }
                let new_x = new_x as usize;
                let new_y = new_y as usize;

                if map[new_x][new_y] != 0 {
                    map[new_x][new_y] += 1;
                }

                if map[new_x][new_y] > 9 {
                    map[new_x][new_y] = 0;
                    flashes.push((new_x, new_y));
                }
            }
        }
        if steps != 0 && i >= steps {
            break;
        }

        flashes_count += step_flashes_count;

        if step_flashes_count >= max_flashes {
            break;
        }

        i += 1;
    }
    visualize_map(&map);
    println!("{}", flashes_count);
    println!("{}", i + 1);
}

fn visualize_map(map: &Vec<Vec<u32>>) {
    for line in map {
        let mut str = String::new();
        for d in line {
            str.push(char::from_digit(*d, 10).unwrap_or('X'));
        }
        println!("{}", str);
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
