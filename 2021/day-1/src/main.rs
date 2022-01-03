use std::io;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut data = vec![];

    while let Ok(n) = stdin.read_line(&mut line) {
        if n == 0 {
            break;
        }

        data.push(line.trim_end().parse().unwrap());
        line.clear();
    }

    println!("{}", part_1(&data));
    println!("{}", part_2(&data, 3));
}

fn part_1(data: &[i32]) -> i32 {
    let mut increased = 0;
    let mut previous_depth = i32::MAX;

    for depth in data {
        let depth = *depth;

        if depth > previous_depth {
            increased += 1;
        }

        previous_depth = depth;
    }

    increased
}

fn part_2(data: &[i32], n: u8) -> i32 {
    let n = n as usize;

    let mut increased = 0;
    let mut previous_depth = i32::MAX;
    let mut group = vec![0; n];
    let mut sum = 0;

    for i in 0..data.len() {
        sum += data[i];

        if i > (n - 1) {
            sum -= group[i % n];
            if sum > previous_depth {
                increased += 1;
            }
            previous_depth = sum;
        } else {
            previous_depth = sum;
        }
        group[i % n] = data[i];
    }

    increased
}