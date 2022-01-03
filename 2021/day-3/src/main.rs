use std::io;

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

fn part_1(data: &Vec<String>) -> u64 {
    if data.is_empty() {
        return 0
    }

    let line_size = data[0].len();

    let mut count = vec![0; line_size];
    
    for line in data {
        let mut pos = 0;
        for c in line.as_bytes() {
            match c {
                b'0' => count[pos] += 1,
                _ => {},
            }
            pos += 1;
        }
    }

    let middle = data.len() / 2;
    let mut gamma = 0;
    let mut epsilon = 0;
    for c in count {
        if c > middle {
            gamma |= 1;
        } else {
            epsilon |= 1;
        }
        gamma <<= 1;
        epsilon <<= 1;
    }
    gamma >>=1;
    epsilon >>=1;
    
    gamma * epsilon
}

fn part_2(data: &Vec<String>) -> u64 {
    if data.is_empty() {
        return 0
    }

    let line_size = data[0].len();

    let mut count = vec![0; line_size];
    let mut middle = data.len() / 2;
    let mut pos = 0;
    let mut prefix = String::new();
    let mut oxygen = data.clone();

    loop {
        for line in oxygen.iter() {
            match line.as_bytes()[pos] {
                b'0' => count[pos] += 1,
                _ => {},
            }
        }

        if count[pos] > middle {
            prefix.push('0');
        } else {
            prefix.push('1');
        }

        oxygen = oxygen.iter().filter(|item| item.starts_with(&prefix)).cloned().collect::<Vec<String>>();

        if oxygen.len() == 1 {
            break;
        }

        pos += 1;
        middle = oxygen.len() / 2;
    }

    let mut count = vec![0; line_size];
    let mut co2 = data.clone();
    middle = data.len() / 2;
    prefix.clear();
    pos = 0;
    loop {
        for line in co2.iter() {
            match line.as_bytes()[pos] {
                b'0' => count[pos] += 1,
                _ => {},
            }
        }

        if count[pos] <= middle {
            prefix.push('0');
        } else {
            prefix.push('1');
        }

        co2 = co2.iter().filter(|item| item.starts_with(&prefix)).cloned().collect::<Vec<String>>();

        if co2.len() == 1 {
            break;
        }

        pos += 1;
        middle = co2.len() / 2;
    }

    (isize::from_str_radix(&oxygen[0], 2).unwrap() * isize::from_str_radix(&co2[0], 2).unwrap()) as u64
}