use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::BufRead;
use std::path::Path;
use std::{env, io};

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

struct ScoredPoint {
    x: usize,
    y: usize,
    score: u32,
}

impl ScoredPoint {
    pub fn point(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl PartialEq<Self> for ScoredPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }
}

impl PartialOrd for ScoredPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Eq for ScoredPoint {}

impl Ord for ScoredPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

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

    let mut full_map = vec![vec![0; map[0].len() * 5]; map.len() * 5];
    for i in 0..5 {
        for j in 0..5 {
            for x in 0..map.len() {
                for y in 0..map[0].len() {
                    full_map[j * map.len() + x][i * map[0].len() + y] =
                        ((map[x][y] + i as u32 + j as u32 - 1) % 9) + 1;
                }
            }
        }
    }

    let goal = (full_map.len() - 1, full_map[0].len() - 1);

    let start = chrono::Utc::now();
    let _path = a_star((0, 0), goal, &full_map);
    let end = chrono::Utc::now();
    let duration = end - start;
    let duration_a = duration.num_nanoseconds().unwrap_or(0) as f64 / 1000000.0;
    println!(
        "A* took {}ms",
        duration.num_nanoseconds().unwrap_or(0) as f64 / 1000000.0
    );

    let start = chrono::Utc::now();
    let _path = dijkstra((0, 0), goal, &full_map);
    let end = chrono::Utc::now();
    let duration = end - start;
    let duration_d = duration.num_nanoseconds().unwrap_or(0) as f64 / 1000000.0;
    println!(
        "Dijkstra took {}ms",
        duration.num_nanoseconds().unwrap_or(0) as f64 / 1000000.0
    );

    println!("{}", duration_d / duration_a)
}

fn a_star(
    start: (usize, usize),
    goal: (usize, usize),
    map: &Vec<Vec<u32>>,
) -> Option<Vec<(usize, usize)>> {
    let mut to_expand: BinaryHeap<Reverse<ScoredPoint>> = BinaryHeap::new();
    let mut to_expand_contents: HashSet<(usize, usize)> = HashSet::new();
    let mut prev: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    let mut g_scores = vec![vec![u32::MAX; map[0].len()]; map.len()];
    let mut f_scores = g_scores.clone();
    g_scores[start.0][start.1] = 0;
    f_scores[start.0][start.1] = 0;
    to_expand.push(Reverse(ScoredPoint {
        x: start.0,
        y: start.1,
        score: manhatten_distance(&start, &goal),
    }));

    prev.insert((start.0, start.1), None);

    to_expand_contents.insert((start.0, start.1));

    while let Some(current) = to_expand.pop() {
        let current = current.0;
        to_expand_contents.remove(&(current.x, current.y));
        if current.point() == goal {
            println!(
                "({},{}) - {}",
                current.x, current.y, g_scores[current.x][current.y]
            );
            return None; // TODO:
        }
        for dir in DIRECTIONS {
            let new_x = current.x as i32 + dir.0;
            let new_y = current.y as i32 + dir.1;
            if new_x < 0 || new_y < 0 || new_x >= map.len() as i32 || new_y >= map[0].len() as i32 {
                continue;
            }

            let new_x = new_x as usize;
            let new_y = new_y as usize;

            let new_score = g_scores[current.x][current.y] + map[new_x][new_y];

            // println!("({},{}) -> {}", new_x, new_y, new_score);

            if new_score < g_scores[new_x][new_y] {
                g_scores[new_x][new_y] = new_score;
                f_scores[new_x][new_y] = new_score + manhatten_distance(&(new_x, new_y), &goal);
                prev.insert((new_x, new_y), Some((current.x, current.y)));
                let point = ScoredPoint {
                    x: new_x,
                    y: new_y,
                    score: f_scores[new_x][new_y],
                };

                if !to_expand_contents.contains(&(new_x, new_y)) {
                    to_expand.push(Reverse(point));
                    to_expand_contents.insert((new_x, new_y));
                }
            }
        }
    }

    None
}

fn dijkstra(start: (usize, usize), goal: (usize, usize), map: &Vec<Vec<u32>>) {
    let mut to_expand: BinaryHeap<Reverse<ScoredPoint>> = BinaryHeap::new();
    let mut adjacent_list = HashMap::new();
    let mut visted: HashSet<(usize, usize)> = HashSet::new();
    let mut prev: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    let mut dist = vec![vec![u32::MAX; map[0].len()]; map.len()];
    dist[start.0][start.1] = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            to_expand.push(Reverse(ScoredPoint {
                x: i,
                y: j,
                score: u32::MAX,
            }));
        }
    }

    for p in &to_expand {
        let current = &p.0;

        for dir in DIRECTIONS {
            let new_x = current.x as i32 + dir.0;
            let new_y = current.y as i32 + dir.1;
            if new_x < 0 || new_y < 0 || new_x >= map.len() as i32 || new_y >= map[0].len() as i32 {
                continue;
            }

            let new_x = new_x as usize;
            let new_y = new_y as usize;

            append_or_insert(
                &mut adjacent_list,
                current.point(),
                ((new_x, new_y), map[new_x][new_y]),
            )
        }
    }

    while let Some(p) = to_expand.pop() {
        let current = p.0;
        if visted.contains(&current.point()) {
            continue;
        }
        visted.insert(current.point());
        if let Some(vec) = adjacent_list.get(&current.point()) {
            for (n, l) in vec {
                let new_dist = dist[current.x][current.y] + l;
                if new_dist < dist[n.0][n.1] {
                    let point = ScoredPoint {
                        x: n.0,
                        y: n.1,
                        score: new_dist,
                    };
                    dist[n.0][n.1] = new_dist;
                    prev.insert(point.point(), Some(current.point()));
                    to_expand.push(Reverse(point));
                }
            }
        }
    }
    println!("{:?}", dist[goal.0][goal.1]);
}

fn append_or_insert<T: Eq + Hash>(
    map: &mut HashMap<T, Vec<((usize, usize), u32)>>,
    k: T,
    v: ((usize, usize), u32),
) {
    if let Some(value) = map.get_mut(&k) {
        value.push(v);
    } else {
        map.insert(k, vec![v]);
    }
}

fn manhatten_distance(a: &(usize, usize), b: &(usize, usize)) -> u32 {
    ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as u32
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
