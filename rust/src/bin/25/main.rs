use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::Iterator;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let lines = read_lines(&args[0])
        .unwrap()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();

    let mut map = vec![vec!['.'; lines.len()]; lines[0].len()];

    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            map[x][y] = c;
        }
    }
    for y in 0..map[0].len() {
        for x in 0..map.len() {
            print!("{}", map[x][y])
        }
        println!();
    }
    println!();

    let mut steps = 0;
    loop {
        steps += 1;
        let mut moved = false;
        for i in 0..2 {
            let mut new_map = map.clone();
            for y in 0..map[0].len() {
                for x in 0..map.len() {
                    if i == 0 && map[x][y] == '>' {
                        let x_new = (x + 1) % map.len();
                        if map[x_new][y] == '.' {
                            new_map[x][y] = '.';
                            new_map[x_new][y] = '>';
                            moved = true;
                        }
                    }
                    if i == 1 && map[x][y] == 'v' {
                        let y_new = (y + 1) % map[0].len();
                        if map[x][y_new] == '.' {
                            new_map[x][y] = '.';
                            new_map[x][y_new] = 'v';
                            moved = true;
                        }
                    }
                }
            }

            map = new_map;
        }

        if !moved {
            break;
        }
    }

    for y in 0..map[0].len() {
        for x in 0..map.len() {
            print!("{}", map[x][y])
        }
        println!();
    }

    println!("{}", steps);
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
