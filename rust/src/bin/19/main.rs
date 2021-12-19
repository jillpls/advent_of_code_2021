use std::collections::HashSet;
use std::env;
use std::fmt::Formatter;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use itertools::Itertools;

const MATRIX: [[[i32; 3]; 3]; 24] = [
    [ [ 1, 0, 0 ], [ 0, 1, 0 ], [ 0, 0, 1 ], ],
    [ [ 1, 0, 0 ], [ 0, 0,-1 ], [ 0, 1, 0 ], ],
    [ [ 1, 0, 0 ], [ 0,-1, 0 ], [ 0, 0,-1 ], ],
    [ [ 1, 0, 0 ], [ 0, 0, 1 ], [ 0,-1, 0 ], ],

    [ [ 0,-1, 0 ], [ 1, 0, 0 ], [ 0, 0, 1 ], ],
    [ [ 0, 0, 1 ], [ 1, 0, 0 ], [ 0, 1, 0 ], ],
    [ [ 0, 1, 0 ], [ 1, 0, 0 ], [ 0, 0,-1 ], ],
    [ [ 0, 0,-1 ], [ 1, 0, 0 ], [ 0,-1, 0 ], ],

    [ [-1, 0, 0 ], [ 0,-1, 0 ], [ 0, 0, 1 ], ],
    [ [-1, 0, 0 ], [ 0, 0,-1 ], [ 0,-1, 0 ], ],
    [ [-1, 0, 0 ], [ 0, 1, 0 ], [ 0, 0,-1 ], ],
    [ [-1, 0, 0 ], [ 0, 0, 1 ], [ 0, 1, 0 ], ],

    [ [ 0, 1, 0 ], [-1, 0, 0 ], [ 0, 0, 1 ], ],
    [ [ 0, 0, 1 ], [-1, 0, 0 ], [ 0,-1, 0 ], ],
    [ [ 0,-1, 0 ], [-1, 0, 0 ], [ 0, 0,-1 ], ],
    [ [ 0, 0,-1 ], [-1, 0, 0 ], [ 0, 1, 0 ], ],

    [ [ 0, 0,-1 ], [ 0, 1, 0 ], [ 1, 0, 0 ], ],
    [ [ 0, 1, 0 ], [ 0, 0, 1 ], [ 1, 0, 0 ], ],
    [ [ 0, 0, 1 ], [ 0,-1, 0 ], [ 1, 0, 0 ], ],
    [ [ 0,-1, 0 ], [ 0, 0,-1 ], [ 1, 0, 0 ], ],

    [ [ 0, 0,-1 ], [ 0,-1, 0 ], [-1, 0, 0 ], ],
    [ [ 0,-1, 0 ], [ 0, 0, 1 ], [-1, 0, 0 ], ],
    [ [ 0, 0, 1 ], [ 0, 1, 0 ], [-1, 0, 0 ], ],
    [ [ 0, 1, 0 ], [ 0, 0,-1 ], [-1, 0, 0 ], ],
];

fn rot((x, y, z): (i32, i32, i32), id: usize) -> (i32, i32, i32) {
        let m = MATRIX[id];
        (
            m[0][0] * x + m[0][1] * y + m[0][2] * z,
            m[1][0] * x + m[1][1] * y + m[1][2] * z,
            m[2][0] * x + m[2][1] * y + m[2][2] * z,
        )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let lines: Vec<String> = read_lines(&args[0]).unwrap().map(|x| x.unwrap()).collect();
    let mut scanner = Vec::new();
    let mut scanners = Vec::new();
    let mut lines = lines.iter();
    lines.next();
    let mut l = lines.next();
    while let Some(line) = l {
        if line.trim().is_empty() {
            scanners.push(scanner.iter().map(|x| *x).collect::<HashSet<(i32, i32, i32)>>());
            scanner = Vec::new();
            lines.next();
            l = lines.next();
            continue;
        }
        let line = line.split(",").collect::<Vec<&str>>();
        let pos = (
            line[0].parse::<i32>().unwrap(),
            line[1].parse::<i32>().unwrap(),
            line[2].parse::<i32>().unwrap(),
        );
        scanner.push(pos);
        l = lines.next();
    }

    scanners.push(scanner.iter().map(|x| *x).collect::<HashSet<(i32, i32, i32)>>());

    let mut final_scanner = scanners.get(0).unwrap().clone();


    println!("{}", final_scanner.len());
    let mut idx = 0;
    let mut visited = HashSet::new();
    visited.insert(0);
    let mut distances = Vec::new();
    loop {
        println!("{}/{}/{}", idx, scanners.len(),visited.len());
        let s = scanners.get(idx).unwrap();
        for i in 0..24 {
            let rotated_s = s.iter().map(| a | rot(*a, i) ).collect::<HashSet<(i32, i32, i32)>>();
            let product = final_scanner.iter().cartesian_product(&rotated_s).map( |(x, y)| (*x, *y)).collect::<HashSet<((i32, i32, i32), (i32, i32, i32))>>();
            let mut differences = product.iter().map( | ((x1, x2, x3), (y1, y2, y3)) | (*x1-*y1, *x2-*y2, *x3-*y3)).counts().iter().map(|(x,y)| (*x, *y)).collect::<Vec<((i32, i32, i32), usize)>>();
            differences.sort_by( | (_, c1), (_,c2)| c1.cmp(c2));
            differences.reverse();
            if differences[0].1 >= 12 {
                let (t1, t2, t3) = differences[0].0;
                let translated_s = rotated_s.iter().map(|(x1, x2, x3)| (x1+t1, x2+t2, x3+t3)).collect::<HashSet<(i32, i32, i32)>>();
                final_scanner = final_scanner.union(&translated_s).map(|x| *x).collect::<HashSet<(i32, i32, i32)>>();
                visited.insert(idx);
                distances.push(differences[0].0);
                break;
            }
        }
        if visited.len() == scanners.len() {
            break;
        }
        idx += 1;
        if idx >= scanners.len() {
            idx = 0;
        }
        while visited.contains(&idx) {
            idx += 1;
            if idx >= scanners.len() {
                idx = 0;
            }
        }
    }

    let mut max = 0;
    let paths = distances.iter().cartesian_product(&distances).collect::<Vec<(&(i32, i32, i32), &(i32, i32, i32))>>();
    for ((x1, x2, x3),(y1,y2,y3)) in paths {
        let len = (y1-x1).abs() + (y2-x2).abs() + (y3-x3).abs();
        if len > max {
            max = len;
        }
    }

    println!("{}", final_scanner.len());
    println!("{}", max);
}

struct Scanner(HashSet<(i32, i32, i32)>);

impl std::fmt::Display for Scanner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        for (x1, x2, x3) in &self.0 {
            str.push_str(&format!("{},{},{}\n", x1, x2, x3));
        }
        write!(f, "{}", str)
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
