use std::io;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut data: Vec<Vec<String>> = vec![];
    let mut first_part: Vec<Vec<String>> = vec![];

    while let Ok(n) = stdin.read_line(&mut line) {
        if n == 0 {
            break;
        }

        let parts: Vec<&str> = line.trim().split('|').collect();
        let segments: Vec<String> = parts[0].split(' ').filter_map(|x| {
            if !x.trim().is_empty() {
                return Some(x.to_owned());
            } else {
                return None;
            }
        }).collect();
        first_part.push(segments);

        let segments: Vec<String> = parts[1].split(' ').filter_map(|x| {
            if !x.trim().is_empty() {
                return Some(x.to_owned());
            } else {
                return None
            }
        }).collect();

        data.push(segments);

        line.clear();
    }

    println!("{}", part_1(&data));
    println!("{}", part_2(&first_part, &data));
}

fn part_1(data: &Vec<Vec<String>>) -> i32 {
    let mut total = 0;
    for part in data {
        for segments in part {
            match segments.len() {
                2 | 3 | 4 | 7 => total += 1,
                _ => (),
            }
        }
    }
    total
}

fn part_2(first_part: &Vec<Vec<String>>, data: &Vec<Vec<String>>) -> i32 {
    let mut total = 0;

    for part in 0..first_part.len() {
        let mut five_segments = vec![];
        let mut six_segments = vec![];
    
        let mut one = None;
        let mut four = None;
        for segments in first_part[part].iter() {
            match segments.len() {
                2 => one = Some(segments),
                4 => four = Some(segments),
                5 => five_segments.push(segments),
                6 => six_segments.push(segments),
                _ => (),
            }
        }

        let mut three = None;
        for pos in 0..five_segments.len() {
            let cur = five_segments[pos];

            let mut is_three = true;
            for c in one.unwrap().as_bytes() {
                if let None = cur.find(*c as char) {
                    is_three = false;
                    break;
                }
            }

            if is_three {
                three = Some(cur);
                five_segments.remove(pos);
                break;
            }
        }

        let mut six = None;
        for pos in 0..six_segments.len() {
            let cur = six_segments[pos];

            let mut is_six = false;
            for c in one.unwrap().as_bytes() {
                if let None = cur.find(*c as char) {
                    is_six = true;
                    break;
                }
            }

            if is_six {
                six = Some(cur);
                six_segments.remove(pos);
                break;
            }
        }

        let mut nine = None;
        for pos in 0..six_segments.len() {
            let cur = six_segments[pos];

            let mut is_nine = true;
            for c in four.unwrap().as_bytes() {
                if let None = cur.find(*c as char) {
                    is_nine = false;
                    break;
                }
            }

            if is_nine {
                nine = Some(cur);
                six_segments.remove(pos);
                break;
            }
        }

        let zero = Some(six_segments[0]);

        let mut five = None;
        let mut two = None;
        for pos in 0..five_segments.len() {
            let cur = five_segments[pos];

            let mut common_segments = 0;
            for c in four.unwrap().as_bytes() {
                if let Some(_) = cur.find(*c as char) {
                    common_segments += 1;
                }
            }

            if common_segments == 3 {
                five = Some(cur);
                five_segments.remove(pos);
                two = Some(five_segments[0]);
                break;
            } else {
                two = Some(cur);
                five_segments.remove(pos);
                five = Some(five_segments[0]);
                break;
            }
        }

        let mut multiplier = 10i32.pow((data[part].len() - 1) as u32);
        let mut current_number = 0;
        for segments in data[part].iter() {
            match segments.len() {
                2 => current_number += 1 * multiplier,
                3 => current_number += 7 * multiplier,
                4 => current_number += 4 * multiplier,
                7 => current_number += 8 * multiplier,
                5 => current_number += get_number(segments, three, five, two) * multiplier,
                6 => current_number += get_number2(segments, zero, six, nine) * multiplier,
                _ => (),
            }
            multiplier /= 10;
        }

        total += current_number;
    }
    total
}

fn get_number(target: &String, three: Option<&String>, five: Option<&String>, _: Option<&String>) -> i32 {
    let mut found = true;
    for c in three.unwrap().as_bytes() {
        if let None = target.find(*c as char) {
            found = false;
            break;
        }
    }
    if found {
        return 3;
    }

    found = true;
    for c in five.unwrap().as_bytes() {
        if let None = target.find(*c as char) {
            found = false;
            break;
        }
    }
    if found {
        return 5;
    }

    2
}

fn get_number2(target: &String, zero: Option<&String>, six: Option<&String>, _: Option<&String>) -> i32 {
    let mut found = true;
    for c in zero.unwrap().as_bytes() {
        if let None = target.find(*c as char) {
            found = false;
            break;
        }
    }
    if found {
        return 0;
    }

    found = true;
    for c in six.unwrap().as_bytes() {
        if let None = target.find(*c as char) {
            found = false;
            break;
        }
    }
    if found {
        return 6;
    }

    9
}