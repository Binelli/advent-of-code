use std::io;

struct Position {
    x: i32,
    y: i32,
}

struct Position2 {
    x: i32,
    y: i32,
    aim: i32,
}

enum Move {
    Forward,
    Down,
    Up,
}

impl Move {
    fn exec_move(&self, position: &mut Position, qty: i32) {
        match self {
            Move::Forward => position.x += qty,
            Move::Down => position.y += qty,
            Move::Up => position.y -= qty,
        }
    }

    fn exec_move2(&self, position: &mut Position2, qty: i32) {
        match self {
            Move::Forward => {
                position.x += qty;
                position.y += position.aim * qty;
            }
            Move::Down => position.aim += qty,
            Move::Up => position.aim -= qty,
        }
    }
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut data: Vec<String> = vec![];

    while let Ok(n) = stdin.read_line(&mut line) {
        if n == 0 {
            break;
        }

        data.push(line.trim_end().to_owned());
        line.clear();
    }

    println!("{}", part_1(&data));
    println!("{}", part_2(&data));
}

fn part_1(data: &Vec<String>) -> i32 {
    let mut position = Position{x: 0, y: 0};

    for direction in data {
        let info: Vec<&str> = direction.split(' ').collect();

        if info.len() < 2 {
            continue;
        }

        let move_dir: Move;
        match info[0] {
            "forward" => move_dir = Move::Forward,
            "down" => move_dir = Move::Down,
            "up" => move_dir = Move::Up,
            _ => continue,
        }

        move_dir.exec_move(&mut position, info[1].parse().unwrap());
    }

    position.x * position.y
}

fn part_2(data: &Vec<String>) -> i32 {
    let mut position = Position2{x: 0, y: 0, aim: 0};

    for direction in data {
        let info: Vec<&str> = direction.split(' ').collect();

        if info.len() < 2 {
            continue;
        }

        let move_dir: Move;
        match info[0] {
            "forward" => move_dir = Move::Forward,
            "down" => move_dir = Move::Down,
            "up" => move_dir = Move::Up,
            _ => continue,
        }

        move_dir.exec_move2(&mut position, info[1].parse().unwrap());
    }

    position.x * position.y
}

