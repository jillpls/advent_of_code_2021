use std::cmp::{max, min};
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let lines = read_lines(&args[0]).unwrap().map(|x| x.unwrap());
    let mut cubes = Vec::new();
    for line in lines {
        let split = line.splitn(2, " ").collect::<Vec<&str>>();
        let on = if split[0] == "on" { true } else { false };
        let xyz = split[1].splitn(3, ",").collect::<Vec<&str>>();
        let xyz = xyz
            .iter()
            .map(|v| {
                (*v).split("=").collect::<Vec<&str>>()[1]
                    .split("..")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();
        let xyz = (
            (xyz[0][0], xyz[0][1]),
            (xyz[1][0], xyz[1][1]),
            (xyz[2][0], xyz[2][1]),
        );
        cubes.push((on, xyz));
    }

    let mut current_cubes: Vec<Cube> = Vec::new();
    for (on, cube2) in cubes {
        let (x, y, z) = cube2;
        if x.0 < -50 || x.1 > 50 || y.0 < -50 || y.1 > 50 || z.0 < -50 || z.1 > 50 {}
        println!("{:?}", cube2);
        let mut new_cubes = Vec::new();
        while let Some(cube1) = current_cubes.pop() {
            new_cubes.append(&mut subtract(cube1, cube2));
        }
        if on {
            new_cubes.push(cube2);
        }
        println!("{:?}", new_cubes.len());
        current_cubes = new_cubes;
    }

    // println!("{:?}", current_cubes[0]);
    let mut sum = 0;
    for (x, y, z) in current_cubes {
        let x_len: u64 = { x.1 - x.0 + 1 } as u64;
        let y_len: u64 = { y.1 - y.0 + 1 } as u64;
        let z_len: u64 = { z.1 - z.0 + 1 } as u64;
        if x_len > 0 && y_len > 0 && z_len > 0 {
            sum += x_len * y_len * z_len;
        }
    }
    println!("{}", sum);
}

fn intersection((x1, y1, z1): Cube, (x2, y2, z2): Cube) -> Option<Cube> {
    if x1.0 > x2.1 || x1.1 < x2.0 || y1.0 > y2.1 || y1.1 < y2.0 || z1.0 > z2.1 || z1.1 < z2.0 {
        return None;
    }
    let intersection = (
        (max(x1.0, x2.0), min(x1.1, x2.1)),
        (max(y1.0, y2.0), min(y1.1, y2.1)),
        (max(z1.0, z2.0), min(z1.1, z2.1)),
    );
    return Some(intersection);
}

fn subtract(cube1: Cube, cube2: Cube) -> Vec<Cube> {
    if let Some(intersection) = intersection(cube1, cube2) {
        let (xi, yi, zi) = intersection;
        let (x, y, z) = cube1;
        let new_cubes = [
            ((x.0, xi.0 - 1), y, z),
            ((xi.1 + 1, x.1), y, z),
            (xi, (y.0, yi.0 - 1), z),
            (xi, (yi.1 + 1, y.1), z),
            (xi, yi, (z.0, zi.0 - 1)),
            (xi, yi, (zi.1 + 1, z.1)),
        ];
        new_cubes
            .iter()
            .filter(|(x, y, z)| x.1 >= x.0 && y.1 >= y.0 && z.1 >= z.0)
            .copied()
            .collect::<Vec<Cube>>()
    } else {
        vec![cube1]
    }
}

type Cube = ((i32, i32), (i32, i32), (i32, i32));

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
