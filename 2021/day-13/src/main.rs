use std::io;
use std::collections::HashSet;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut points = HashSet::new();
    let mut fold: Vec<(i32, i32)> = vec![];

    while let Ok(_) = stdin.read_line(&mut line) {
        if line.trim().is_empty() {
            break;
        }

        let coordinate: Vec<i32> = line.trim().split(',').map(|x| x.parse::<i32>().unwrap()).collect();

        points.insert((coordinate[0], coordinate[1]));

        line.clear();
    }

    while let Ok(n) = stdin.read_line(&mut line) {
        if n == 0 {
            break;
        }

        let tokens: Vec<&str> = line.trim().split('=').collect();

        if tokens.len() == 2 {
            let axis = &tokens[0][tokens[0].len() - 1..];

            let axis = if axis == "x" {
                0
            } else {
                1
            };

            let pos = tokens[1].parse::<i32>().unwrap();

            fold.push((axis, pos));
        }

        line.clear();
    }

    println!("{}", fold_sheet(&fold, &points, 1));
    println!("{}", fold_sheet(&fold, &points, fold.len() as i32));
}

fn fold_on_x(pos: i32, points: &HashSet<(i32,i32)>) -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();

    for (x, y) in points.iter() {
        if x > &pos {
            let target_coord = (pos - (x - pos), *y);

            result.insert(target_coord);
        } else {
            result.insert((*x, *y));
        }
    }

    result
}

fn fold_on_y(pos: i32, points: &HashSet<(i32,i32)>) -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();

    for (x, y) in points.iter() {
        if y > &pos {
            let target_coord = (*x, pos - (y - pos));

            result.insert(target_coord);
        } else {
            result.insert((*x, *y));
        }
    }

    result
}

fn fold_sheet(fold: &Vec<(i32, i32)>, points: &HashSet<(i32, i32)>, qty: i32) -> i32 {
    let mut points = points.clone();

    for (axis, pos) in fold.iter().take(qty as usize) {
        if axis == &0 {
            points = fold_on_x(*pos, &points);
        } else {
            points = fold_on_y(*pos, &points);
        }
    }

    for y in 0..6 {
        for x in 0..40 {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!("-");
            }
        }
        println!("");
    }
    points.len() as i32
}
