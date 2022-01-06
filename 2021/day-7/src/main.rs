use std::io;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut data: Vec<i32> = vec![];

    if let Ok(_) = stdin.read_line(&mut line) {
        data = line.trim().split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    }

    println!("{}", part_1(&data));
    println!("{}", part_2(&data));
}

fn part_1(data: &Vec<i32>) -> i32 {
    let max_horizontal_pos = data.iter().max().unwrap();
    let mut min_fuel_cost = i32::MAX;
    
    for pos in 0..*max_horizontal_pos {
        let mut curr_fuel_cost = 0;
        for crab_pos in data {
            curr_fuel_cost += (crab_pos - pos).abs();
            if curr_fuel_cost >= min_fuel_cost {
                break;
            }
        }

        min_fuel_cost = min_fuel_cost.min(curr_fuel_cost);
    }

    min_fuel_cost
}

fn part_2(data: &Vec<i32>) -> i32 {
    let max_horizontal_pos = data.iter().max().unwrap();
    let mut min_fuel_cost = i32::MAX;
    let mut fuel_costs = vec![0; (*max_horizontal_pos + 1) as usize];

    for step in 1..=*max_horizontal_pos {
        fuel_costs[step as usize] = fuel_costs[(step - 1) as usize] + step;
    }

    for pos in 0..*max_horizontal_pos {
        let mut curr_fuel_cost = 0;
        for crab_pos in data {
            curr_fuel_cost += fuel_costs[(crab_pos - pos).abs() as usize];
            if curr_fuel_cost >= min_fuel_cost {
                break;
            }
        }

        min_fuel_cost = min_fuel_cost.min(curr_fuel_cost);
    }
    
    min_fuel_cost
}