use std::cmp::{max, min};
use std::collections::HashMap;
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

    let mut vent_lines = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for line in lines {
        let line: String = line.unwrap();
        let (start, end) = {
            let line: Vec<&str> = line.split(" -> ").collect();
            (String::from(line[0]), String::from(line[1]))
        };
        let vent_line = (get_coordinates(&start), get_coordinates(&end));
        if vent_line.0 .0 > max_x {
            max_x = vent_line.0 .0;
        }
        if vent_line.1 .0 > max_x {
            max_x = vent_line.1 .0;
        }
        if vent_line.0 .1 > max_y {
            max_y = vent_line.0 .1;
        }
        if vent_line.1 .1 > max_y {
            max_y = vent_line.1 .1;
        }
        vent_lines.push(vent_line);
    }
    let mut true_grid = vec![vec![0; max_x as usize + 1]; max_y as usize + 1];
    let mut grid: HashMap<(u32, u32), u32> = HashMap::new();
    let mut count = 0;
    for line in vent_lines {
        let start = line.0;
        let end = line.1;
        let start_x = start.0 as i64;
        let end_x = end.0 as i64;
        let start_y = start.1 as i64;
        let end_y = end.1 as i64;

        println!("({},{}) -> ({},{})", start_x, start_y, end_x, end_y);
        let s_e_x: i64 = (end_x - start_x) as i64;
        let s_e_y: i64 = (end_y - start_y) as i64;

        let dir_x = if s_e_x == 0 { 0 } else { s_e_x / s_e_x.abs() };
        let dir_y = if s_e_y == 0 { 0 } else { s_e_y / s_e_y.abs() };

        let len = max(
            ((end_x - start_x) as i64).abs(),
            ((end_y - start_y) as i64).abs(),
        );
        println!("{}", len);
        for i in 0..len + 1 {
            let x = (start_x + i * dir_x) as u32;
            let y = (start_y + i * dir_y) as u32;
            true_grid[x as usize][y as usize] += 1;
            if let Some(v) = grid.get_mut(&(x, y)) {
                *v += 1;
                if *v == 2 {
                    count += 1;
                }
            } else {
                grid.insert((x, y), 1);
            }
        }
        // for x in start_x..end_x+1 {
        //     for y in start_y..end_y+1 {
        //         true_grid[x as usize][y as usize] += 1;
        //         if let Some(v) = grid.get_mut(&(x,y)) {
        //             *v += 1;
        //             if *v == 2 {
        //                 count += 1;
        //             }
        //         } else {
        //             grid.insert((x, y), 1);
        //         }
        //     }
        // }
    }
    for v in true_grid {}
    println!("{}", count);
}

fn get_coordinates(val: &str) -> (u32, u32) {
    let val: Vec<&str> = val.split(',').collect();
    (
        val[0].parse::<u32>().unwrap(),
        val[1].parse::<u32>().unwrap(),
    )
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
