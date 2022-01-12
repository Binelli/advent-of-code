use std::io;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    let mut data: Vec<Vec<u8>> = vec![];

    while let Ok(n) = stdin.read_line(&mut line) {
        if n == 0 {
            break;
        }

        let mut heights = vec![];
        for c in line.trim().as_bytes() {
            heights.push(c - b'0');
        }

        data.push(heights);

        line.clear();
    }

    println!("{}", part_1(&data));
    println!("{}", part_2(&data));
}

fn part_1(heights_map: &Vec<Vec<u8>>) -> i32 {
    let mut result = 0i32;

    for (row, heights) in heights_map.iter().enumerate() {
        for (col, height) in heights.iter().enumerate() {
            if col > 0 {
                if height >= &heights[col - 1] {
                    continue;
                }
            }

            if col < (heights.len() - 1) {
                if height >= &heights[col + 1] {
                    continue;
                }
            }

            if row > 0 {
                if height >= &heights_map[row - 1][col] {
                    continue;
                }
            }

            if row < (heights_map.len() - 1) {
                if height >= &heights_map[row + 1][col] {
                    continue;
                }
            }

            result += (*height + 1) as i32;
        }
    }

    result
}

fn part_2(heights_map: &Vec<Vec<u8>>) -> i32 {
    let mut basins = vec![];
    let mut basin_map = HashMap::new();

    for (row, heights) in heights_map.iter().enumerate() {
        for (col, height) in heights.iter().enumerate() {
            if *height == 9 {
                continue;
            }

            let coordinate = (row, col);

            if basin_map.contains_key(&coordinate) {
                continue;
            }

            // new basin
            let basin_no = basins.len();
            basins.push(0);

            let mut queue = VecDeque::new();

            queue.push_back(coordinate);

            // println!("Analyzing start at: {:?}", coordinate);

            // queue to coordinates that are part of same basin - start with just discovered coordinate
            while !queue.is_empty() {
                // pop item
                let coordinate = queue.pop_front().unwrap();
                let (row, col) = coordinate;

                let height = heights_map[row][col] as i8;

                // check if not part of map
                if basin_map.contains_key(&coordinate) {
                    continue;
                }

                basins[basin_no] += 1;
                basin_map.insert(coordinate, basin_no);

                // check west
                if col > 0 {
                    let west_height = heights_map[row][col - 1] as i8;
                    if (west_height != 9) && ((height < west_height) || (height > west_height)) {
                        let west_coordinate = (row, col - 1);

                        queue.push_back(west_coordinate);
                    }
                }
    
                // check north
                if row > 0 {
                    let north_height = heights_map[row - 1][col] as i8;
                    if (north_height != 9) && ((height < north_height) || (height > north_height)) {
                        let north_coordinate = (row - 1, col);

                        queue.push_back(north_coordinate);
                    }
                }

                // check east
                if col < (heights.len() - 1) {
                    let east_height = heights_map[row][col + 1] as i8;
                    if (east_height != 9) && ((height < east_height) || (height > east_height)) {
                        let east_coordinate = (row, col + 1);

                        queue.push_back(east_coordinate);
                    }
                }

                // check south
                if row < (heights_map.len() - 1) {
                    let south_height = heights_map[row + 1][col] as i8;
                    if (south_height != 9) && ((height < south_height) || (height > south_height)) {
                        let south_coordinate = (row + 1, col);

                        queue.push_back(south_coordinate);
                    }
                }
            }
        }
    }

    basins.sort();
 
    basins[basins.len() - 1] * basins[basins.len() - 2] * basins[basins.len() - 3]
}