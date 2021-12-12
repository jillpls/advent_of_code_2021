use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];
    let lines: Vec<String> = read_lines(&args[0]).unwrap().map(|x| x.unwrap()).collect();
    let mut connections = Vec::new();
    for l in lines {
        let split: Vec<&str> = l.split("-").collect();
        let (a, b) = (split[0], split[1]);
        connections.push((a.to_string(), b.to_string()));
    }

    let mut nodes: Vec<Node> = vec![];
    let mut node_map: HashMap<String, usize> = HashMap::new();

    for (from, to) in connections.iter() {
        let from_idx = get_index(from, &mut nodes, &mut node_map);
        let to_idx = get_index(to, &mut nodes, &mut node_map);
        nodes[from_idx].0.push(to_idx);
        nodes[to_idx].0.push(from_idx);
    }

    let mut start = 0;
    let mut end = 0;

    for (i, (_, n)) in nodes.iter().enumerate() {
        if n.as_str() == "start" {
            start = i;
        } else if n.as_str() == "end" {
            end = i;
        }
    }

    let paths = get_paths(start, end, &nodes, Vec::new(), false);
    for p in &paths {
        let mut str = String::new();
        for name in p.iter().map(|i| &nodes.get(*i).unwrap().1) {
            str.push_str(name);
            if name.as_str() != "end" {
                str.push_str("->");
            }
        }
        println!("{}", str);
    }

    println!("{}", paths.len());
}

fn get_index(name: &str, nodes: &mut Vec<Node>, map: &mut HashMap<String, usize>) -> usize {
    if let Some(i) = map.get(name) {
        *i
    } else {
        nodes.push((vec![], name.to_string()));
        let idx = nodes.len() - 1;
        map.insert(name.to_string(), idx);
        idx
    }
}

fn get_paths(
    start: usize,
    end: usize,
    nodes: &Vec<Node>,
    mut visited: Vec<usize>,
    visited_twice: bool,
) -> Vec<Vec<usize>> {
    visited.push(start);
    let mut paths = Vec::new();
    for t in &nodes.get(start).unwrap().0 {
        let mut visited_twice_t = visited_twice;
        let name = &nodes.get(*t).unwrap().1;

        if name.as_str() == "start" {
            continue;
        }

        if name.chars().all(|x| x.is_lowercase()) {
            if visited.contains(t) {
                if visited_twice || name.as_str() == "start" || name.as_str() == "end" {
                    continue;
                } else {
                    visited_twice_t = true;
                }
            }
        }
        if t == &end {
            let mut path = visited.clone();
            path.push(end);
            paths.push(path);
            continue;
        }
        paths.append(&mut get_paths(
            *t,
            end,
            nodes,
            visited.clone(),
            visited_twice_t,
        ));
    }
    paths
}

type Node = (Vec<usize>, String);

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
