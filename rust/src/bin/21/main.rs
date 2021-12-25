use std::cmp::{max, min};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

const WINNING_SCORE: i32 = 21;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let mut lines = read_lines(&args[0]).unwrap().map(|x| x.unwrap());
    let init_pos1 = lines
        .next()
        .unwrap()
        .rsplit(" ")
        .next()
        .unwrap()
        .parse::<i32>()
        .unwrap()
        - 1;
    let init_pos2 = lines
        .next()
        .unwrap()
        .rsplit(" ")
        .next()
        .unwrap()
        .parse::<i32>()
        .unwrap()
        - 1;

    let mut pos1 = init_pos1;
    let mut pos2 = init_pos2;

    let mut i = 2;
    let mut score1 = 0;
    let mut score2 = 0;
    loop {
        if i % 2 == 0 {
            pos1 += i * 3;
            pos1 = pos1 % 10;
            score1 += pos1 + 1;
        } else {
            pos2 += i * 3;
            pos2 = pos2 % 10;
            score2 += pos2 + 1;
        }

        if score1 >= 1000 || score2 >= 1000 {
            break;
        }
        i += 3;
    }
    println!("{}", (i + 1) * min(score1, score2));

    let game_state = ((init_pos1, 0), (init_pos2, 0), false);

    let winners = get_winners(&game_state, &mut HashMap::new());

    println!("{:?}", max(winners.0, winners.1));
}

fn get_winners(
    prev_game_state: &GameState,
    cache: &mut HashMap<GameState, (u64, u64)>,
) -> (u64, u64) {
    let mut winners = (0, 0);
    for r1 in 1..4 {
        for r2 in 1..4 {
            for r3 in 1..4 {
                let ((mut pos1, mut score1), (mut pos2, mut score2), player) = prev_game_state;
                if !player {
                    pos1 += r1 + r2 + r3;
                    pos1 = pos1 % 10;
                    score1 += pos1 + 1;
                } else {
                    pos2 += r1 + r2 + r3;
                    pos2 = pos2 % 10;
                    score2 += pos2 + 1;
                }
                let game_state = ((pos1, score1), (pos2, score2), !*player);

                if score1 >= WINNING_SCORE {
                    winners.0 += 1;
                } else if score2 >= WINNING_SCORE {
                    winners.1 += 1;
                } else {
                    let rec_winners = if let Some(cached) = cache.get(&game_state) {
                        *cached
                    } else {
                        let w = get_winners(&game_state, cache);
                        cache.insert(game_state, w);
                        w
                    };
                    winners.0 += rec_winners.0;
                    winners.1 += rec_winners.1;
                }
            }
        }
    }
    winners
}

type GameState = ((i32, i32), (i32, i32), bool);

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
