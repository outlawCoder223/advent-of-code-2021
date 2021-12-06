use std::fs;

const BOARD_LEN: usize = 5;
const BOARD_WID: usize = 5;
const MARKED: i16 = -1;
const BINGO: i16 = -5;

#[derive(Debug, Clone)]
struct BingoBoard {
    board: Vec<Vec<i16>>,
    bingo: bool,
    counted: bool,
    score: usize,
}

impl BingoBoard {
    fn new() -> BingoBoard {
        BingoBoard {
            board: vec![vec![0; BOARD_WID]; BOARD_LEN],
            bingo: false,
            counted: false,
            score: 0,
        }
    }

    fn check_board(&mut self, call_num: i16) -> bool {
        if self.bingo {
            return self.bingo;
        }
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j] == call_num {
                    self.board[i][j] = MARKED;
                    let mut sum_col: i16 = 0;
                    for k in 0..self.board.len() {
                        sum_col += self.board[k][j];
                    }
                    let sum_row = self.board[i].iter().sum::<i16>();
                    if sum_col == BINGO || sum_row == BINGO {
                        self.bingo = true;
                    }
                }
            }
        }

        if self.bingo {
            self.calculate_score(call_num);
        }

        self.bingo
    }

    fn calculate_score(&mut self, win_num: i16) {
        let sum: i16 = self
            .board
            .iter()
            .flat_map(|row| row.iter())
            .cloned()
            .filter(|num| num.is_positive())
            .sum();

        self.score = sum as usize * win_num as usize;
    }
}

#[derive(Debug)]
struct BingoData {
    call_nums: Vec<i16>,
    boards: Vec<BingoBoard>,
    winner: bool,
    winning_score: usize,
    last_score: usize,
}

impl BingoData {
    fn new(bingo_input: &str) -> BingoData {
        let mut lines = bingo_input.lines().filter(|&l| !l.is_empty()).into_iter();

        let call_nums = lines
            .next()
            .unwrap()
            .split(",")
            .map(|n| n.parse::<i16>().unwrap())
            .collect();

        let mut line_ctr = 0;
        let mut board_count = 0;

        let mut boards = Vec::new();

        lines.for_each(|line| {
            if line_ctr == 0 {
                boards.push(BingoBoard::new());
            }
            let nums: Vec<i16> = line
                .split_whitespace()
                .filter_map(|n| n.parse::<i16>().ok())
                .collect();

            boards[board_count].board[line_ctr] = nums;
            line_ctr += 1;

            if line_ctr >= 5 {
                line_ctr = 0;
                board_count += 1;
            }
        });

        BingoData {
            call_nums,
            boards,
            winner: false,
            winning_score: 0,
            last_score: 0,
        }
    }

    fn play(&mut self) {
        let call_nums = self.call_nums.iter();

        let mut winning_order: Vec<BingoBoard> = Vec::new();

        for num in call_nums {
            for board in self.boards.iter_mut() {
                if board.check_board(*num) {
                    if !board.counted {
                        winning_order.push(board.clone());
                        board.counted = true;
                    }
                } else {
                    continue;
                }
            }
        }
        let winning_board = winning_order.first().unwrap();
        self.winning_score = winning_board.score;
        let last_board = winning_order.last().unwrap();
        self.last_score = last_board.score;

        println!("Winning Score: {}", self.winning_score);
        println!("Last Score: {}", self.last_score);
    }
}

pub fn execute() {
    println!("---------DAY 4--------");
    let bingo_input = fs::read_to_string("input/input_04.txt").expect("Unable to open file");

    let mut bingo_data = BingoData::new(bingo_input.as_str());

    bingo_data.play();
}
