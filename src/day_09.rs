use std::fs;

fn find_low_spots(digits: &Vec<Vec<u32>>) -> (u32, Vec<(usize, usize)>) {
    let height = digits.len();
    let width = digits[0].len();

    let mut coords: Vec<(usize, usize)> = Vec::new();
    let mut risk = 0;
    let mut right: u32;
    let mut left: u32;
    let mut top: u32;
    let mut bottom: u32;
    let mut curr: u32;

    for y in 0..height {
        for x in 0..width {
            curr = digits[y][x];
            if x == 0 {
                if y == 0 {
                    right = digits[y][x + 1];
                    bottom = digits[y + 1][x];

                    if curr < right && curr < bottom {
                        coords.push((x, y));
                        risk += curr + 1;
                    }
                } else if y == height - 1 {
                    right = digits[y][x + 1];
                    top = digits[y - 1][x];

                    if curr < right && curr < top {
                        coords.push((x, y));
                        risk += curr + 1;
                    }
                } else {
                    top = digits[y - 1][x];
                    right = digits[y][x + 1];
                    bottom = digits[y + 1][x];

                    if curr < right && curr < bottom && curr < top {
                        coords.push((x, y));
                        risk += curr + 1;
                    }
                }
            } else if x == width - 1 {
                if y == 0 {
                    left = digits[y][x - 1];
                    bottom = digits[y + 1][x];

                    if curr < left && curr < bottom {
                        coords.push((x, y));
                        risk += curr + 1;
                    }
                } else if y == height - 1 {
                    left = digits[y][x - 1];
                    top = digits[y - 1][x];

                    if curr < left && curr < top {
                        coords.push((x, y));
                        risk += curr + 1;
                    }
                } else {
                    top = digits[y - 1][x];
                    left = digits[y][x - 1];
                    bottom = digits[y + 1][x];

                    if curr < left && curr < bottom && curr < top {
                        coords.push((x, y));
                        risk += curr + 1;
                    }
                }
            } else {
                left = digits[y][x - 1];
                right = digits[y][x + 1];
                if y == 0 {
                    bottom = digits[y + 1][x];

                    if curr < left && curr < bottom && curr < right {
                        coords.push((x, y));
                        risk += curr + 1;
                    }
                } else if y == height - 1 {
                    top = digits[y - 1][x];

                    if curr < left && curr < top && curr < right {
                        coords.push((x, y));
                        risk += curr + 1;
                    }
                } else {
                    bottom = digits[y + 1][x];
                    top = digits[y - 1][x];

                    if curr < left && curr < top && curr < right && curr < bottom {
                        coords.push((x, y));
                        risk += curr + 1;
                    }
                }
            }
        }
    }

    (risk, coords)
}

fn is_in_bounds(x: i32, y: i32, basin: &Vec<Vec<u32>>) -> bool {
    let height = basin.len() as i32;
    let width = basin[0].len() as i32;

    x >= 0 && y >= 0 && x < width && y < height
}

fn is_valid_space(x: i32, y: i32, basin: &Vec<Vec<u32>>) -> bool {
    is_in_bounds(x, y, basin) && basin[y as usize][x as usize] < 9
}

fn find_basin(x: i32, y: i32, basin: &mut Vec<Vec<u32>>) -> u32 {
    if !is_valid_space(x, y, basin) {
        return 0;
    } else {
        basin[y as usize][x as usize] = 9;

        let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        return 1 + dirs.iter().fold(0, |count, (_y, _x)| {
            return count + find_basin(x + _x, y + _y, basin);
        });
    }
}

pub fn execute() {
    println!("---------DAY 9----------");
    let input = fs::read_to_string("input/input_09.txt").expect("Unable to read file");

    let mut digits: Vec<Vec<u32>> = input
        .lines()
        .map(|d| d.chars().map(|d| d.to_digit(10).unwrap()).collect())
        .collect();

    let (risk, coords) = find_low_spots(&digits);

    let mut basin_sizes = Vec::new();

    for (x, y) in coords.iter() {
        basin_sizes.push(find_basin(*x as i32, *y as i32, &mut digits))
    }

    basin_sizes.sort();
    basin_sizes.reverse();

    let result = &basin_sizes[0..3].iter().fold(1, |acc, x| acc * x);

    println!("Risk: {}", risk);
    println!("Result: {}", result);
}
