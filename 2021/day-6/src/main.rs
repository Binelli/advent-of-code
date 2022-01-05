use std::io;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut data: Vec<i32> = vec![];

    if let Ok(_) = stdin.read_line(&mut line) {
        data = line.trim().split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    }

    println!("{}", part_1(&mut data.clone(), 80));
    println!("{}", part_2(&mut data, 256));
}

fn part_1(data: &mut Vec<i32>, days: i32) -> i32 {
    let mut new_fishes = 0;

    for _ in 0..days {
        for _ in 0..new_fishes {
            data.push(9);
        }
        new_fishes = 0;
        
        for timer in data.iter_mut() {
            if timer == &0 {
                *timer = 6;
            } else {
                *timer -= 1;
                if *timer == 0 {
                    new_fishes += 1;
                }
            }
        }
    }

    data.len() as i32
}

fn part_2(data: &mut Vec<i32>, days: i32) -> u64 {
    let mut total = 0 as u64;

    for fish in data {
        let mut days_vector = vec![vec![0 as u64; 2]; days as usize];

        let fish = *fish as usize;
        days_vector[fish][0] += 1;
        days_vector[fish][1] += 1;

        total += 2;

        for day in 0..days {
            if (day + 7) < days {
                let qty = days_vector[day as usize][0];
                days_vector[(day + 7) as usize][0] += qty;
                days_vector[(day + 7) as usize][1] += qty;
                total += qty;
            }
            if (day + 9) < days {
                let qty = days_vector[day as usize][1];
                days_vector[(day + 9) as usize][0] += qty;
                days_vector[(day + 9) as usize][1] += qty;
                total += qty;
            }
        }
    }

    total
}