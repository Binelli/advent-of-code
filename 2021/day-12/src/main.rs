use std::io;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug)]
enum CaveType {
    Start,
    End,
    Big,
    Small,
}

#[derive(Debug)]
struct Cave {
    cave_type: CaveType,
    connections: Vec<usize>,
}

impl Cave {
    fn new(cave_name: &str) -> Cave {
        let mut cave_type = CaveType::Small;

        match cave_name {
            "start" => cave_type = CaveType::Start,
            "end" => cave_type = CaveType::End,
            _ => {
                if cave_name.to_uppercase() == cave_name {
                    cave_type = CaveType::Big;
                }
            }
        }

        Cave {
            cave_type,
            connections: vec![]
        }
    }

    fn add_connection(&mut self, connected_cave: usize) {
        self.connections.push(connected_cave);
    }

    fn is_end(&self) -> bool {
        match self.cave_type {
            CaveType::End => true,
            _ => false,
        }
    }

    fn is_big(&self) -> bool {
        match self.cave_type {
            CaveType::Big => true,
            _ => false,
        }
    }

    fn get_connections(&self) -> &[usize] {
        &self.connections[..]
    }
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut caves: Vec<Cave> = vec![];
    let mut caves_map: HashMap<String, usize> = HashMap::new();

    while let Ok(n) = stdin.read_line(&mut line) {
        if n == 0 {
            break;
        }

        let caves_names: Vec<&str> = line.trim().split('-').collect();

        build_caves(&caves_names, &mut caves, &mut caves_map);

        line.clear();
    }

    let mut visited = HashSet::new();
    println!("{}", paths(&caves, *caves_map.get("start").unwrap(), *caves_map.get("end").unwrap(), &mut visited));

    let mut visited = HashMap::new();
    println!("{}", paths_part2(&caves, *caves_map.get("start").unwrap(), *caves_map.get("end").unwrap(), &mut visited, false));
}

fn paths(caves: &Vec<Cave>, start: usize, end: usize, visited: &mut HashSet<usize>) -> i32 {
    let mut result = 0;

    let start_cave = &caves[start];
    for connection in start_cave.get_connections() {
        let next_cave = &caves[*connection];
        if next_cave.is_end() {
            result += 1;
        } else if next_cave.is_big() || !visited.contains(connection) {
            visited.insert(*connection);
            result += paths(caves, *connection, end, visited);
            visited.remove(connection);
        }
    }

    result
}

fn paths_part2(caves: &Vec<Cave>, start: usize, end: usize, visited: &mut HashMap<usize, u8>, small_visited_twice: bool) -> i32 {
    let mut result = 0;

    let start_cave = &caves[start];
    for connection in start_cave.get_connections() {
        let next_cave = &caves[*connection];
        if next_cave.is_end() {
            result += 1;
        } else if !next_cave.is_big() {
            if !visited.contains_key(connection) {
                visited.insert(*connection, 1);
                result += paths_part2(caves, *connection, end, visited, small_visited_twice);
                visited.remove(connection);
            }
            else if !small_visited_twice {
                result += paths_part2(caves, *connection, end, visited, true);
            }
        } else {
            result += paths_part2(caves, *connection, end, visited, small_visited_twice);
        }
    }

    result
}

fn build_caves(caves_names: &Vec<&str>, caves: &mut Vec<Cave>, caves_map: &mut HashMap<String, usize>) {
    if caves_names.len() == 2 {
        let l_cave = caves_names[0].to_owned();
        let r_cave = caves_names[1].to_owned();

        if !caves_map.contains_key(&l_cave) {
            caves_map.insert(l_cave.clone(), caves.len());
            caves.push(Cave::new(&l_cave));
        }

        if !caves_map.contains_key(&r_cave) {
            caves_map.insert(r_cave.clone(), caves.len());
            caves.push(Cave::new(&r_cave));
        }

        let l_cave_idx = caves_map.get(&l_cave).unwrap();
        let r_cave_idx = caves_map.get(&r_cave).unwrap();

        if l_cave != "start" {
            caves[*r_cave_idx].add_connection(*l_cave_idx);
        }

        if r_cave != "start" {
            caves[*l_cave_idx].add_connection(*r_cave_idx);
        }
    }
}

