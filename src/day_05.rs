use std::fs;

#[derive(Debug)]
struct Vents {
    plane: Vec<Vec<usize>>,
    coords: Vec<Line>,
}

impl Vents {
    fn new() -> Vents {
        Vents {
            plane: vec![vec![0; 1000]; 1000],
            coords: Vec::new(),
        }
    }

    fn scan_vents(&mut self) {
        for coord in self.coords.iter() {
            Self::draw_vent_line(&mut self.plane, &coord);
        }
    }

    fn print_vents(&self) {
        for row in self.plane.iter() {
            println!("{:?}", row);
        }
    }

    fn get_overlaps(&self) -> usize {
        let mut overlap_count = 0;

        for row in self.plane.iter() {
            for &col in row {
                if col > 1 {
                    overlap_count += 1;
                }
            }
        }

        overlap_count
    }

    fn draw_vent_line(plane: &mut Vec<Vec<usize>>, coord: &Line) {
        match coord.direction {
            Direction::Horizontal => {
                for x in coord.x1..(coord.x2 + 1) {
                    plane[coord.y1][x] += 1;
                }
            }
            Direction::Vertical => {
                for y in coord.y1..(coord.y2 + 1) {
                    plane[y][coord.x1] += 1;
                }
            }
            Direction::Diagonal => {
                let positive = coord.y1 <= coord.y2;

                let mut x = coord.x1;
                let mut y = coord.y1;

                while x <= coord.x2 {
                    plane[y][x] += 1;
                    x += 1;
                    if positive {
                        y += 1;
                    } else {
                        if y > 0 {
                            y -= 1;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
}

impl Direction {
    fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Direction {
        if x1 == x2 {
            return Direction::Vertical;
        } else if y1 == y2 {
            return Direction::Horizontal;
        } else {
            Direction::Diagonal
        }
    }
}

#[derive(Debug, Clone)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    direction: Direction,
}
impl Line {
    fn new(coords: Vec<usize>) -> Line {
        let mut x1 = coords[0];
        let mut y1 = coords[1];
        let mut x2 = coords[2];
        let mut y2 = coords[3];

        let direction = Direction::new(x1, y1, x2, y2);
        match direction {
            Direction::Diagonal | Direction::Horizontal => match x1 <= x2 {
                false => {
                    let mut temp = x1;
                    x1 = x2;
                    x2 = temp;
                    temp = y1;
                    y1 = y2;
                    y2 = temp;
                }
                _ => {}
            },
            _ => match y1 <= y2 {
                false => {
                    let mut temp = x1;
                    x1 = x2;
                    x2 = temp;
                    temp = y1;
                    y1 = y2;
                    y2 = temp;
                }
                _ => {}
            },
        }

        Line {
            x1,
            x2,
            y1,
            y2,
            direction,
        }
    }
}

fn parse_line(input: &str) -> Line {
    let new_line = input.replace(" -> ", ",");
    let coords: Vec<usize> = new_line
        .split(",")
        .into_iter()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    Line::new(coords)
}

pub fn execute() {
    println!("--------DAY 5--------");
    let input = fs::read_to_string("input/input_05.txt").expect("Unable to read file");

    let mut vents = Vents::new();

    let input_lines: Vec<&str> = input.lines().collect::<Vec<&str>>();
    input_lines
        .iter()
        .for_each(|line| vents.coords.push(parse_line(line)));

    vents.scan_vents();
    // vents.print_vents();
    println!("Overlaps: {}", vents.get_overlaps());
}
