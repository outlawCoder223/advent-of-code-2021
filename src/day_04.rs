use std::fs;

const BOARD_LEN: usize = 5;
const BOARD_WID: usize = 5;
const MARKED: i16 = -1;
const BINGO: i16 = -5;

#[derive(Debug)]
struct BingoBoard {
    board: Vec<Vec<i16>>,
    bingo: bool,
}

impl BingoBoard {
    fn new() -> BingoBoard {
        BingoBoard {
            board: vec![vec![0; BOARD_WID]; BOARD_LEN],
            bingo: false,
        }
    }

    fn check_board(&mut self, call_num: i16) -> bool {
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

        self.bingo
    }

    fn calculate_score(&self, win_num: i16) -> i16 {
        let sum: i16 = self
            .board
            .iter()
            .flat_map(|row| row.iter())
            .cloned()
            .filter(|num| num.is_positive())
            .sum();

        sum * win_num
    }
}

#[derive(Debug)]
struct BingoData {
    call_nums: Vec<i16>,
    boards: Vec<BingoBoard>,
    winner: bool,
    score: usize,
}
impl BingoData {
    fn new() -> BingoData {
        BingoData {
            call_nums: Vec::new(),
            boards: Vec::new(),
            winner: false,
            score: 0,
        }
    }

    fn play(&mut self) {
        let mut call_nums = self.call_nums.iter();

        while !self.winner {
            let pick = call_nums.next();

            match pick {
                None => {
                    println!("No one won?");
                    break;
                }
                Some(n) => {
                    for board in self.boards.iter_mut() {
                        if board.check_board(*n) {
                            self.winner = true;
                            self.score = board.calculate_score(*n) as usize;
                            println!("{:?}", board.board);
                        }
                    }
                }
            }
        }
        if self.winner {
            println!("Winning Score: {}", self.score);
        }
    }
}

pub fn execute() {
    println!("---------DAY 4--------");
    let input = fs::read_to_string("input/input_04.txt").expect("Unable to open file");
    let mut lines = input.lines().filter(|&l| !l.is_empty()).into_iter();

    let mut bingo_data = BingoData::new();

    bingo_data.call_nums = lines
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<i16>().unwrap())
        .collect();

    let mut line_ctr = 0;
    let mut board_count = 0;

    lines.for_each(|line| {
        if line_ctr == 0 {
            bingo_data.boards.push(BingoBoard::new());
        }
        let nums: Vec<i16> = line
            .split_whitespace()
            .filter_map(|n| n.parse::<i16>().ok())
            .collect();

        bingo_data.boards[board_count].board[line_ctr] = nums;
        line_ctr += 1;
        if line_ctr >= 5 {
            line_ctr = 0;
            board_count += 1;
        }
    });

    bingo_data.play();
    // println!("{:?}", bingo_data.boards);
}
