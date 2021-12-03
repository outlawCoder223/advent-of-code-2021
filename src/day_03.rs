use std::fs;
const REPORT_BIN_LEN: usize = 12;

#[derive(Debug)]
struct Report<'a> {
    binary_report: &'a Vec<usize>,
    num_ones: [usize; REPORT_BIN_LEN],
    len: usize,
    gamma: usize,
    epsilon: usize,
    pwr_consump: usize,
}
impl Report<'_> {
    fn new(len: usize, binary_report: &Vec<usize>) -> Report {
        Report {
            binary_report,
            num_ones: [0; REPORT_BIN_LEN],
            len,
            gamma: 0,
            epsilon: 0,
            pwr_consump: 0,
        }
    }

    fn calculate_epsilon(&mut self) {
        self.epsilon = self.gamma ^ 0xFFF;
    }

    fn calculate_pwr_consump(&mut self) {
        self.pwr_consump = self.gamma * self.epsilon;
    }
}

pub fn execute() {
    let input = fs::read_to_string("input/input_03.txt").expect("Unable to open file");
    let binary_nums: Vec<usize> = input
        .lines()
        .map(|n| usize::from_str_radix(n, 2).unwrap())
        .collect();

    let mut report = Report::new(binary_nums.len(), &binary_nums);

    for num in report.binary_report {
        for i in 0..REPORT_BIN_LEN {
            report.num_ones[i] += num >> (REPORT_BIN_LEN - 1 - i) & 0x1;
        }
    }

    report.num_ones.iter().for_each(|num| {
        if num >= &(report.len / 2) {
            report.gamma = report.gamma << 1 | 0x1;
        } else {
            report.gamma = report.gamma << 1;
        }
    });

    report.calculate_epsilon();
    report.calculate_pwr_consump();

    println!("Gamma: {}", report.gamma);
    println!("Epsilon: {}", report.epsilon);
    println!("Power Consumption: {}", report.pwr_consump);
}
