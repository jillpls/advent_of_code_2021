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

    let mut parses = Vec::new();
    let mut current_parse: Vec<String> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            parses.push(current_parse.clone());
            current_parse = Vec::new();
        } else {
            current_parse.push(line);
        }
    }
    if !current_parse.is_empty() {
        parses.push(current_parse);
    }

    let numbers = &parses[0];
    let boards = &parses[1..];
    let (mut boards, map) = parse_boards(boards);
    let numbers: Vec<u16> = numbers[0]
        .split(',')
        .map(|x| x.parse::<u16>().unwrap())
        .collect();

    let mut boards_clone = boards.clone();

    let mut result = (0, 0);
    'outer: for n in numbers.clone() {
        if let Some(v) = map.get(&n) {
            println!("{} drawn", n);
            for (i, x, y) in v {
                boards[*i][*x][*y].1 = true;
                if check_board(&boards[*i], *x, *y) {
                    println!("Board {} won", i);
                    result = (n, *i);
                    break 'outer;
                }
            }
        }
    }

    let mut sum = 0;
    for v in &boards[result.1] {
        for w in v {
            if !w.1 {
                sum += w.0;
            }
        }
    }

    let mut boards = boards_clone;

    let mut winners: HashMap<usize, ()> = HashMap::new();
    let mut result = (0, 0);
    'outer: for n in numbers {
        if let Some(v) = map.get(&n) {
            println!("{} drawn", n);
            for (i, x, y) in v {
                boards[*i][*x][*y].1 = true;
                if check_board(&boards[*i], *x, *y) {
                    if !winners.contains_key(i) {
                        println!("Board {} won", i);
                        winners.insert(*i, ());
                        result = (n, *i);
                        if winners.len() == boards.len() {
                            break 'outer;
                        }
                    }
                }
            }
        }
    }

    let mut sum = 0;
    for v in &boards[result.1] {
        for w in v {
            if !w.1 {
                sum += w.0;
            }
        }
    }

    println!("{} * {} = {}", sum, result.0, sum * result.0);
}

fn check_board(board: &Vec<Vec<(u16, bool)>>, x: usize, y: usize) -> bool {
    let mut row = true;
    let mut col = true;

    for i in 0..5 {
        if !board[x][i].1 {
            col = false;
        }
        if !board[i][y].1 {
            row = false;
        }
    }
    return row || col;
}

fn parse_boards(
    boards: &[Vec<String>],
) -> (
    Vec<Vec<Vec<(u16, bool)>>>,
    HashMap<u16, Vec<(usize, usize, usize)>>,
) {
    let mut parsed_boards = Vec::new();
    let mut map: HashMap<u16, Vec<(usize, usize, usize)>> = HashMap::new();
    for (i, b) in boards.iter().enumerate() {
        let (board, numbers) = parse_board(b);
        for (n, x, y) in numbers {
            if let Some(v) = map.get_mut(&n) {
                v.push((i, x, y));
            } else {
                map.insert(n, vec![(i, x, y)]);
            }
        }
        parsed_boards.push(board);
    }
    (parsed_boards, map)
}

fn parse_board(board: &Vec<String>) -> (Vec<Vec<(u16, bool)>>, Vec<(u16, usize, usize)>) {
    let mut parsed_board = vec![vec![(0, false); 5]; 5];
    let mut numbers = Vec::new();
    for (i, s) in board.iter().enumerate() {
        for (j, n) in s.split_whitespace().into_iter().enumerate() {
            let number = n.parse::<u16>().unwrap();
            numbers.push((number, i, j));
            parsed_board[i][j] = (number, false);
        }
    }
    (parsed_board, numbers)
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
