use std::io;
use std::collections::VecDeque;

#[derive(Clone)]
struct Octopus {
    energy: u32,
    flashed: bool,
}

impl Octopus {
    fn new(energy: u32) -> Octopus {
        Octopus {energy, flashed: false}
    }

    fn increase_energy(&mut self, energy_increase: u32) -> bool {
        self.energy += energy_increase;

        if !self.flashed && self.energy >= 10 {
            self.flashed = true;
            return true;
        }

        false
    }

    fn reset(&mut self) {
        if self.flashed {
            self.flashed = false;
            self.energy = 0;
        }
    }
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut data: Vec<Vec<Octopus>> = vec![];

    while let Ok(n) = stdin.read_line(&mut line) {
        if n == 0 {
            break;
        }

        let mut octopus_row = vec![];
        for c in line.trim().chars() {
            let energy: u32 = c.to_digit(10).unwrap();

            octopus_row.push(Octopus::new(energy));
        }
        data.push(octopus_row);

        line.clear();
    }

    println!("{}", part_1(&mut data.clone(), 100));
    println!("{}", part_2(&mut data));
}

fn part_1(octopus_matrix: &mut Vec<Vec<Octopus>>, steps: i32) -> i32 {
    let rows = octopus_matrix.len();
    let cols = octopus_matrix[0].len();

    let mut flashes = 0;

    for _ in 0..steps {
        let mut increased_by_neighboors: VecDeque<(usize, usize)> = VecDeque::new();

        for row in 0..rows {
            for col in 0..cols {
                let octopus = &mut octopus_matrix[row][col];

                if octopus.increase_energy(1) {
                    flashes += 1;

                    if col > 0 {
                        if row > 0 {
                            increased_by_neighboors.push_back((row - 1, col - 1));
                        }

                        increased_by_neighboors.push_back((row, col - 1));

                        if row < (rows - 1) {
                            increased_by_neighboors.push_back((row + 1, col - 1));
                        }
                    }

                    if row > 0 {
                        increased_by_neighboors.push_back((row - 1, col));
                    }

                    if row < (rows - 1) {
                        increased_by_neighboors.push_back((row + 1, col));
                    }

                    if col < (cols - 1) {
                        if row > 0 {
                            increased_by_neighboors.push_back((row - 1, col + 1));
                        }

                        increased_by_neighboors.push_back((row, col + 1));

                        if row < (rows - 1) {
                            increased_by_neighboors.push_back((row + 1, col + 1));
                        }
                    }
                }
            }
        }

        while !increased_by_neighboors.is_empty() {
            let (row, col) = increased_by_neighboors.pop_front().unwrap();

            let octopus = &mut octopus_matrix[row][col];

            if octopus.increase_energy(1) {
                flashes += 1;

                if col > 0 {
                    if row > 0 {
                        increased_by_neighboors.push_back((row - 1, col - 1));
                    }

                    increased_by_neighboors.push_back((row, col - 1));

                    if row < (rows - 1) {
                        increased_by_neighboors.push_back((row + 1, col - 1));
                    }
                }

                if row > 0 {
                    increased_by_neighboors.push_back((row - 1, col));
                }

                if row < (rows - 1) {
                    increased_by_neighboors.push_back((row + 1, col));
                }

                if col < (cols - 1) {
                    if row > 0 {
                        increased_by_neighboors.push_back((row - 1, col + 1));
                    }

                    increased_by_neighboors.push_back((row, col + 1));

                    if row < (rows - 1) {
                        increased_by_neighboors.push_back((row + 1, col + 1));
                    }
                }
            }
        }

        for row in 0..rows {
            for col in 0..cols {
                let octopus = &mut octopus_matrix[row][col];
                octopus.reset();
            }
        }
    }

    flashes
}

fn part_2(octopus_matrix: &mut Vec<Vec<Octopus>>) -> i32 {
    let rows = octopus_matrix.len();
    let cols = octopus_matrix[0].len();

    let mut step = 1;
    loop {
        let mut increased_by_neighboors: VecDeque<(usize, usize)> = VecDeque::new();

        let mut flashes = 0;
        for row in 0..rows {
            for col in 0..cols {
                let octopus = &mut octopus_matrix[row][col];

                if octopus.increase_energy(1) {
                    flashes += 1;

                    if col > 0 {
                        if row > 0 {
                            increased_by_neighboors.push_back((row - 1, col - 1));
                        }

                        increased_by_neighboors.push_back((row, col - 1));

                        if row < (rows - 1) {
                            increased_by_neighboors.push_back((row + 1, col - 1));
                        }
                    }

                    if row > 0 {
                        increased_by_neighboors.push_back((row - 1, col));
                    }

                    if row < (rows - 1) {
                        increased_by_neighboors.push_back((row + 1, col));
                    }

                    if col < (cols - 1) {
                        if row > 0 {
                            increased_by_neighboors.push_back((row - 1, col + 1));
                        }

                        increased_by_neighboors.push_back((row, col + 1));

                        if row < (rows - 1) {
                            increased_by_neighboors.push_back((row + 1, col + 1));
                        }
                    }
                }
            }
        }

        while !increased_by_neighboors.is_empty() {
            let (row, col) = increased_by_neighboors.pop_front().unwrap();

            let octopus = &mut octopus_matrix[row][col];

            if octopus.increase_energy(1) {
                flashes += 1;

                if col > 0 {
                    if row > 0 {
                        increased_by_neighboors.push_back((row - 1, col - 1));
                    }

                    increased_by_neighboors.push_back((row, col - 1));

                    if row < (rows - 1) {
                        increased_by_neighboors.push_back((row + 1, col - 1));
                    }
                }

                if row > 0 {
                    increased_by_neighboors.push_back((row - 1, col));
                }

                if row < (rows - 1) {
                    increased_by_neighboors.push_back((row + 1, col));
                }

                if col < (cols - 1) {
                    if row > 0 {
                        increased_by_neighboors.push_back((row - 1, col + 1));
                    }

                    increased_by_neighboors.push_back((row, col + 1));

                    if row < (rows - 1) {
                        increased_by_neighboors.push_back((row + 1, col + 1));
                    }
                }
            }
        }

        for row in 0..rows {
            for col in 0..cols {
                let octopus = &mut octopus_matrix[row][col];
                octopus.reset();
            }
        }

        if flashes == (rows * cols) {
            break;
        }

        step += 1;
    }

    step
}