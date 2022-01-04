use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut data: Vec<Line> = vec![];

    while let Ok(n) = stdin.read_line(&mut line) {
        if n == 0 {
            break;
        }

        let mut points: Vec<Point> = line.trim().split(' ').enumerate().filter_map(|(idx, s)| {
            if idx == 1 {
                return None;
            }
            let coordinates: Vec<i32> = s.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
            Some(Point {x: coordinates[0], y: coordinates[1]})
        }).collect();
        data.push(Line {start: points.remove(0), end: points.remove(0)});

        line.clear();
    }

    println!("{}", part_1(&data));
    println!("{}", part_2(&data));
}

fn part_1(data: &Vec<Line>) -> i32 {
    let mut marked_points = HashMap::new();

    for line in data {
        if line.start.x == line.end.x {
            let range;
            if line.start.y < line.end.y {
                range = line.start.y..=line.end.y;
            } else {
                range = line.end.y..=line.start.y;
            }

            for y in range {
                let point = Point {x: line.start.x, y: y};

                let qty = *marked_points.get(&point).unwrap_or(&0);
                marked_points.insert(point, qty + 1);
            }
        } else if line.start.y == line.end.y {
            let range;
            if line.start.x < line.end.x {
                range = line.start.x..=line.end.x;
            } else {
                range = line.end.x..=line.start.x;
            }

            for x in range {
                let point = Point {x: x, y: line.start.y};

                let qty = *marked_points.get(&point).unwrap_or(&0);
                marked_points.insert(point, qty + 1);
            }
        }
    }

    marked_points.iter().filter(|(_, v)| **v >= 2).count() as i32
}

fn part_2(data: &Vec<Line>) -> i32 {
    let mut marked_points = HashMap::new();

    for line in data {
        if line.start.x == line.end.x {
            let range;
            if line.start.y < line.end.y {
                range = line.start.y..=line.end.y;
            } else {
                range = line.end.y..=line.start.y;
            }

            for y in range {
                let point = Point {x: line.start.x, y: y};

                let qty = *marked_points.get(&point).unwrap_or(&0);
                marked_points.insert(point, qty + 1);
            }
        } else if line.start.y == line.end.y {
            let range;
            if line.start.x < line.end.x {
                range = line.start.x..=line.end.x;
            } else {
                range = line.end.x..=line.start.x;
            }

            for x in range {
                let point = Point {x: x, y: line.start.y};

                let qty = *marked_points.get(&point).unwrap_or(&0);
                marked_points.insert(point, qty + 1);
            }
        } else {
            let mut x_start = line.start.x;
            let x_end = line.end.x;
            let mut y_start = line.start.y;
            let y_end = line.end.y;

            while x_start != x_end {
                let point = Point {x: x_start, y: y_start};

                let qty = *marked_points.get(&point).unwrap_or(&0);
                marked_points.insert(point, qty + 1);

                if y_start < y_end {
                    y_start += 1;
                } else {
                    y_start -= 1;
                }

                if x_start < x_end {
                    x_start += 1;
                } else {
                    x_start -= 1;
                }
            }

            let point = Point {x: x_start, y: y_start};

            let qty = *marked_points.get(&point).unwrap_or(&0);
            marked_points.insert(point, qty + 1);
        }
    }

    marked_points.iter().filter(|(_, v)| **v >= 2).count() as i32
}

