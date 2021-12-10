use std::fs;

pub fn execute() {
    let input = fs::read_to_string("input/input_09.txt").expect("Unable to read file");

    let digits: Vec<Vec<u32>> = input
        .lines()
        .map(|d| d.chars().map(|d| d.to_digit(10).unwrap()).collect())
        .collect();

    let height = digits.len();
    let width = digits[0].len();
    println!("{}, {}", height, width);

    let mut risk = 0;
    let mut right: u32;
    let mut left: u32;
    let mut top: u32;
    let mut bottom: u32;
    let mut curr: u32;

    // for digit in digits.iter() {
    //     println!("{:?}", digit);
    // }

    for y in 0..height {
        for x in 0..width {
            curr = digits[y][x];
            if x == 0 {
                if y == 0 {
                    right = digits[y][x + 1];
                    bottom = digits[y + 1][x];

                    if curr < right && curr < bottom {
                        println!("{}, {}", x, y);
                        risk += curr + 1;
                    }
                } else if y == height - 1 {
                    right = digits[y][x + 1];
                    top = digits[y - 1][x];

                    if curr < right && curr < top {
                        println!("{}, {}", x, y);
                        risk += curr + 1;
                    }
                } else {
                    top = digits[y - 1][x];
                    right = digits[y][x + 1];
                    bottom = digits[y + 1][x];

                    if curr < right && curr < bottom && curr < top {
                        println!("{}, {}", x, y);
                        risk += curr + 1;
                    }
                }
            } else if x == width - 1 {
                if y == 0 {
                    left = digits[y][x - 1];
                    bottom = digits[y + 1][x];

                    if curr < left && curr < bottom {
                        println!("{}, {}", x, y);
                        risk += curr + 1;
                    }
                } else if y == height - 1 {
                    left = digits[y][x - 1];
                    top = digits[y - 1][x];

                    if curr < left && curr < top {
                        println!("{}, {}", x, y);
                        risk += curr + 1;
                    }
                } else {
                    top = digits[y - 1][x];
                    left = digits[y][x - 1];
                    bottom = digits[y + 1][x];

                    if curr < left && curr < bottom && curr < top {
                        println!("{}, {}", x, y);
                        risk += curr + 1;
                    }
                }
            } else {
                left = digits[y][x - 1];
                right = digits[y][x + 1];
                if y == 0 {
                    bottom = digits[y + 1][x];

                    if curr < left && curr < bottom && curr < right {
                        println!("{}, {}", x, y);
                        risk += curr + 1;
                    }
                } else if y == height - 1 {
                    top = digits[y - 1][x];

                    if curr < left && curr < top && curr < right {
                        println!("{}, {}", x, y);
                        risk += curr + 1;
                    }
                } else {
                    bottom = digits[y + 1][x];
                    top = digits[y - 1][x];

                    if curr < left && curr < top && curr < right && curr < bottom {
                        println!("{}, {}", x, y);
                        risk += curr + 1;
                    }
                }
            }
        }
    }

    println!("{}", risk);
}
