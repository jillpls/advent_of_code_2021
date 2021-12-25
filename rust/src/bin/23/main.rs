use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Debug, Clone, Copy, Default, Hash)]
struct Amphipod {
    pub kind : char,
    pub cost : u32,
    pub target : usize,
    pub position: (usize, usize),
    pub move_count : usize,
}

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

const CAVES : [[(usize, usize);4];4] = [
[(3,2), (3,3), (3,4), (3,5)],
[(5,2), (5,3), (5,4), (5,5)],
[(7,2), (7,3), (7,4), (7,5)],
[(9,2), (9,3), (9,4), (9,5)],
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let lines = read_lines(&args[0]).unwrap().map(|x| x.unwrap()).collect::<Vec<String>>();
    let mut map = [[' '; 7]; 13];
    let mut possible_targets = HashSet::new();
    let mut amphipods = [Amphipod::default(); 16];
    let mut i = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            map[x][y] = char;
            if char == '.' && ! [3, 5, 7, 9].contains(&x) {
                possible_targets.insert((x, y));
            }
            if ['A', 'B', 'C', 'D'].contains(&char) {
                let pos = (x, y);
                let r = match char {
                    'A' => {
                        Some(('A',1,1))
                    }
                    'B' => {
                        Some(('B',10,2))
                    }
                    'C' => {
                        Some(('C',100,3))
                    }
                    'D' => {
                        Some(('D',1000,4))
                    }
                    _ => {
                        None
                    }
                };
                if let Some(r) = r {

                    amphipods[i] = Amphipod {
                        kind: r.0,
                        cost: r.1,
                        target: r.2,
                        position: pos,
                        move_count: 0
                    };
                    i+=1;
                }
            }
        }
    }

    for i in 0..amphipods.len() {
        if amphipods[i].position.0 == CAVES[amphipods[i].target - 1][0].0 {
            let mut is_correct = true;
            for y in amphipods[i].position.1..CAVES[amphipods[i].target - 1][3].1+1 {
                if map[amphipods[i].position.0][y] != amphipods[i].kind {
                    is_correct = false;
                }
            }
            if is_correct {
                amphipods[i].move_count = 2;
                println!("Already correct: {:?}", amphipods[i]);
            }
        }
    }


    let mut game = Game {
        game_tree: vec![
            GameTree {
                state: GameState { map, amphipods, depth : 0 },
                pos: 0,
                parent: None,
                children: vec![],
                cost : 0,
                min_cost : u64::MAX,
            }
        ]
    };

    let mut cache_map = HashMap::new();
    cache_map.insert(map, 0);

    let mut winner = 0;

    let mut min_cost = u64::MAX;

    let mut to_compute = VecDeque::new();
    to_compute.push_back(0);
        'outer: while let Some(current_index) = to_compute.pop_back() {
            let (cost, mut new_states) = {
                let mut current_node = &mut game.game_tree[current_index];
                let cost = current_node.cost;
                let mut current_state = current_node.state;
                (cost, current_state.expand(&possible_targets))
            };

            let new_states : HashSet<(GameState, u64)> = HashSet::from_iter(new_states.iter().copied());

            for (s, new_cost) in &new_states {
                if s.is_winning() && (cost+new_cost) < min_cost {
                    min_cost = cost + new_cost
                }
            }

            for (s, new_cost) in new_states {

                let cost = cost + new_cost;

                if cost > min_cost {
                    continue;
                }

                if let Some(c) = cache_map.get_mut(&s.map) {
                    if *c > cost {
                        *c = cost;
                    } else {
                        continue;
                    }
                } else {
                    cache_map.insert(s.map, cost);
                }

                let len = game.game_tree.len();
                to_compute.push_back(len);
                game.game_tree[current_index].children.push(
                    len
                );
                game.game_tree.push(
                    GameTree {
                        state: s,
                        pos: len,
                        parent: Some(current_index),
                        children: vec![],
                        cost,
                        min_cost
                    }
                );
                if s.is_winning() {
                    println!("{}", cost);
                    winner = len;

                    to_compute.pop_back();
                    if cost < min_cost {
                        let mut pos = Some(game.game_tree.len() - 1);
                        while let Some(p) = pos {
                            game.game_tree[p].min_cost = cost;
                            pos = game.game_tree[p].parent;
                        }
                    }
                }
            }
        }
    if winner != 0 {
        print_moves(winner, &game.game_tree);
    }
}

fn get_cave_move(a : &Amphipod, map: &[[char;7];13]) -> Option<(usize, usize)> {
    let cave = CAVES[a.target - 1];
    let mut cave_pos = 4;
    while cave_pos > 0 {
        let cave_spot = cave[cave_pos-1];
        if map[cave_spot.0][cave_spot.1] == '.' {
            return Some(cave_spot);
        } else if map[cave_spot.0][cave_spot.1] == a.kind {
            cave_pos -= 1;
            continue;
        } else {
            return None;
        }
    }
    None
}

#[derive(Default, Copy, Clone, Hash)]
struct GameState {
    pub map : [[char;7];13],
    amphipods : [Amphipod;16],
    depth : u32
}

impl PartialEq<Self> for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map && self.depth == other.depth
    }
}

impl Eq for GameState {}

impl GameState {

    fn min_cost(&self) -> u64 {
        let mut cost = 0;
        for a in self.amphipods {
            if a.position.0 != a.target*2+1 {
                cost += a.cost as u64 * ((a.target*2+1) as i32 - a.position.0 as i32).abs() as u64;
                cost += a.cost as u64 * (a.position.1-1 + 1) as u64;
            }
        }
        cost
    }

    pub fn is_winning(&self) -> bool {
        for a in self.amphipods {
            if a.move_count < 2 {
                return false;
            }
        }
        true
    }

    pub fn expand(&self, possible_targets: &HashSet<(usize, usize)> ) -> Vec<(GameState, u64)>{
        let mut amphipods = self.amphipods;
        amphipods.sort_by(|x, y | {x.cost.cmp(&y.cost)});
        amphipods.reverse();
        let map = self.map;

        let mut new_states = Vec::new();

        for mut j in 0..2 {
            for (i, a) in amphipods.iter().enumerate() {
                if a.move_count >= 2 {
                    continue;
                }

                let mut skip = true;
                for d in DIRECTIONS {
                    if is_passable(map[(a.position.0 as i32 + d.0) as usize][(a.position.1 as i32 + d.1) as usize]) {
                        skip = false;
                    }
                }
                if skip {
                    continue;
                }

                let mut move_count = 0;
                let mut possible_targets_a = if let Some(cave_pos) = get_cave_move(a, &map) {
                    move_count = 2;
                    j = 2;
                    vec![cave_pos]
                } else {
                    if j == 1 {
                        continue;
                    }
                    if a.move_count == 0 {
                        move_count = 1;
                        possible_targets.iter().copied().collect::<Vec<(usize, usize)>>()
                    } else {
                        continue;
                    }
                };
                possible_targets_a.sort();
                for p in possible_targets_a {
                    let cost = get_cost(a.position, p, &map);
                    if cost.is_none() {
                        continue;
                    }
                    let cost = cost.unwrap() as u64;
                    let mut new_state = GameState {
                        map,
                        amphipods,
                        depth : self.depth + 1,
                    };

                    new_state.map[a.position.0][a.position.1] = '.';
                    new_state.map[p.0][p.1] = a.kind;
                    new_state.amphipods[i].position = p;
                    new_state.amphipods[i].move_count = move_count;
                    if !new_state.is_losing() {
                        new_states.push((new_state, cost * (a.cost as u64)));
                    }
                }
            }
        }
        new_states
    }

    fn is_losing(&self) -> bool {
        false
    }
}

fn print_moves(pos: usize, game_tree: &Vec<GameTree>) {
    let orig_pos = pos;
    let mut pos = Some(pos);


    let mut path = Vec::new();

    while let Some(n) = pos {
        path.push(n);
        pos = game_tree[n].parent;
    }

    for i in path.iter().rev() {
        print_map(&game_tree[*i].state.map);
        println!();
    }
}

#[derive(Default)]
struct Game {
    pub game_tree : Vec<GameTree>,
}

#[derive(Default, Clone)]
struct GameTree {
    state : GameState,
    pos : usize,
    parent: Option<usize>,
    children: Vec<usize>,
    cost: u64,
    min_cost: u64,
}

fn print_map(map: &[[char;7];13]) {
    for y in 0..map[0].len() {
        for x in 0..map.len() {
            print!("{}", map[x][y]);
        }
        println!();
    }
}

fn get_cost(start: (usize, usize), end : (usize, usize), map: &[[char; 7];13]) -> Option<u32> {
    if !is_passable(map[end.0][end.1]) {
        return None;
    }
    let orig_start = start;
    let start = if start.0 > end.0 {
        (start.0-1, start.1)
    } else {
        (start.0+1, start.1)
    };
    for i in min(start.0, end.0)..max(start.0, end.0)+1 {
        if map[i][1] != '.' {
            return None;
        }
    }
    return Some(((orig_start.1 - 1) + (orig_start.0 as i32 - end.0 as i32).abs() as usize + (end.1-1)) as u32);
}

fn is_passable(char : char) -> bool {
    if char == '.' || char == ',' {
        true
    } else {
        false
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