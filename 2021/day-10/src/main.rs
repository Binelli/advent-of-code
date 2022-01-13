use std::io;
use std::collections::VecDeque;
use std::collections::HashMap;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut data: Vec<String> = vec![];

    while let Ok(n) = stdin.read_line(&mut line) {
        if n == 0 {
            break;
        }

        data.push(line.trim().to_owned());

        line.clear();
    }

    println!("{}", part_1(&data));
    println!("{}", part_2(&data));
}

fn part_1(lines: &Vec<String>) -> i32 {
    let char_map = HashMap::from([
        (b'(', b')'),
        (b'[', b']'),
        (b'{', b'}'),
        (b'<', b'>'),
    ]);

    let char_score = HashMap::from([
        (b')', 3),
        (b']', 57),
        (b'}', 1197),
        (b'>', 25137),
    ]);

    let mut result = 0;
    for line in lines {
        let mut stack: VecDeque<u8> = VecDeque::new();

        for c in line.as_bytes() {
            match  c {
                b'(' | b'[' | b'{' | b'<' => stack.push_back(*c),
                b')' | b']' | b'}' | b'>' => {
                    let last_open = stack.pop_back().unwrap();
                    if *c != char_map[&last_open] {
                        result += char_score[c];
                        break;
                    }
                },
                _ => panic!("Invalid character"),
            }
        }
    }

    result
}

fn part_2(lines: &Vec<String>) -> i64 {
    let char_map = HashMap::from([
        (b'(', b')'),
        (b'[', b']'),
        (b'{', b'}'),
        (b'<', b'>'),
    ]);

    let char_score = HashMap::from([
        (b'(', 1),
        (b'[', 2),
        (b'{', 3),
        (b'<', 4),
    ]);

    let mut results = vec![];
    for line in lines {
        let mut stack: VecDeque<u8> = VecDeque::new();
        let mut result: i64 = 0;

        let mut corrupted_line = true;
        for c in line.as_bytes() {
            match  c {
                b'(' | b'[' | b'{' | b'<' => stack.push_back(*c),
                b')' | b']' | b'}' | b'>' => {
                    let last_open = stack.pop_back().unwrap();
                    if *c != char_map[&last_open] {
                        corrupted_line = false;
                        break;
                    }
                },
                _ => panic!("Invalid character"),
            }
        }

        if corrupted_line {
            while !stack.is_empty() {
                let last_open = stack.pop_back().unwrap();

                result *= 5;
                result += char_score[&last_open];
            }
            results.push(result);
        }
    }

    results.sort();
    results[results.len() / 2]
}