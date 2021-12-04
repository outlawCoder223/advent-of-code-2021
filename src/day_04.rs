use std::fs;

#[derive(Debug)]
struct BingoBoard {
    board: Vec<Vec<u8>>,
    bingo: bool,
}

impl BingoBoard {
    fn new() -> BingoBoard {
        BingoBoard {
            board: vec![vec![0; 5]; 5],

            bingo: false,
        }
    }
}

pub fn execute() {
    let input = fs::read_to_string("input/input_04.txt").expect("Unable to open file");
    let mut lines = input.lines().filter(|&l| !l.is_empty()).into_iter();

    let call_nums: Vec<u8> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<u8>().unwrap())
        .collect();

    println!("{:?}", call_nums);

    let mut boards: Vec<BingoBoard> = Vec::new();

    let mut line_ctr = 0;
    let mut board_count = 0;

    lines.for_each(|line| {
        if line_ctr == 0 {
            boards.push(BingoBoard::new());
        }
        let nums: Vec<u8> = line
            .split_whitespace()
            .filter_map(|n| n.parse::<u8>().ok())
            .collect();

        boards[board_count].board[line_ctr] = nums;
        line_ctr += 1;
        if line_ctr >= 5 {
            line_ctr = 0;
            board_count += 1;
        }
    });

    println!("{:?}", boards);
}
