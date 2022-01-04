use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

#[derive(Clone)]
struct Board {
    size: i8,
    numbers: HashMap<i32, i32>,
    row_checked: Vec<i32>,
    col_checked: Vec<i32>,
}

impl Board {
    fn new(size: i8, numbers: &[i32]) -> Board {
        let mut board = Board {
            size,
            numbers: HashMap::new(),
            row_checked: vec![0; size as usize],
            col_checked: vec![0; size as usize],
        };

        if numbers.len() == ((size * size) as usize) {
            for (idx, &number) in numbers.iter().enumerate() {
                board.numbers.insert(number, idx as i32);
            }
        }

        board
    }

    fn check(&mut self, number: i32) -> bool {
        if self.numbers.contains_key(&number) {
            let &position = self.numbers.get(&number).unwrap();

            let size = self.size as i32;

            let row = (position / size) as usize;
            self.row_checked[row] += 1;
            if self.row_checked[row] == size {
                return true;
            }

            let col = (position % size) as usize;
            self.col_checked[col] += 1;
            if self.col_checked[col] == size {
                return true
            }
        }

        false
    }

    fn get_result(&self, last_number: i32, numbers: &[i32]) -> i32 {
        let mut result = 0;
        let mut hash_set = HashSet::new();
        for number in numbers {
            hash_set.insert(number);
        }

        for &number in self.numbers.keys() {
            if !hash_set.contains(&number) {
                result += number;
            }
        }

        result * last_number
    }
}

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut line).expect("failed to read");
    let picked_numbers: Vec<i32> = line.trim_end().split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    line.clear();

    let mut boards: Vec<Board> = vec![];

    while let Ok(n) = stdin.read_line(&mut line) {
        if n == 0 {
            break;
        }

        line.clear();

        let mut board_numbers: Vec<i32> = vec![];
        for _ in 0..5 {
            stdin.read_line(&mut line).expect("failed to read");
            let mut line_numbers: Vec<i32> = line.trim().split(' ').filter_map(|x| {
                let x = x.trim();
                if x.is_empty() {
                    return None;
                }
                return Some(x.trim().parse::<i32>().unwrap());
            }).collect();

            board_numbers.append(&mut line_numbers);
            line.clear();
        }
        
        let board = Board::new(5, &board_numbers);
        boards.push(board);
    }

    println!("{}", part_1(&mut boards.clone(), &picked_numbers));
    println!("{}", part_2(&mut boards, &picked_numbers));
}

fn part_1(boards: &mut Vec<Board>, picked_numbers: &[i32]) -> i32 {
    for (idx, picked_number) in picked_numbers.iter().enumerate() {
        for board in boards.iter_mut() {
            if board.check(*picked_number) {
                return board.get_result(*picked_number, &picked_numbers[0..idx + 1]);
            }
        }
    }
    0
}

fn part_2(boards: &mut Vec<Board>, picked_numbers: &[i32]) -> i32 {
    let mut closed_boards = HashSet::new();
    let boards_qty = boards.len();

    for (idx, picked_number) in picked_numbers.iter().enumerate() {
        for (board_idx, board) in boards.iter_mut().enumerate() {
            if !closed_boards.contains(&board_idx) && board.check(*picked_number) {
                if closed_boards.len() == (boards_qty - 1) {
                    return board.get_result(*picked_number, &picked_numbers[0..idx + 1]);
                } else {
                    closed_boards.insert(board_idx);
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::Board;

    #[test]
    fn row_winner() {
        let mut board = Board::new(3, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);

        board.check(4);
        board.check(5);
        assert_eq!(board.check(6), true);
    }

    #[test]
    fn col_winner() {
        let mut board = Board::new(3, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);

        board.check(3);
        board.check(6);
        assert_eq!(board.check(9), true);
    }

    #[test]
    fn no_diag_winner() {
        let mut board = Board::new(3, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);

        board.check(1);
        board.check(5);
        assert_eq!(board.check(9), false);
    }

    #[test]
    fn check_result() {
        let mut board = Board::new(5, &[14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13,  6,  5, 2, 0, 12,  3,  7]);

        let picked_numbers = [7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24];

        for number in picked_numbers {
            board.check(number);
        }

        assert_eq!(board.get_result(24, &picked_numbers), 4512);
    }
}
