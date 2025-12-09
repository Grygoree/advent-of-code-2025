fn main() {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

pub fn part1(input: &str) -> u32 {
    let mut result = 0;
    for l in input.lines() {
        let bank: Vec<u32> = l.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let mut tens_digit = 0;
        let mut ones_digit = 0;

        for i in 0..(bank.len() - 1) {
            if bank[i] > tens_digit {
                tens_digit = bank[i];
                ones_digit = bank[i + 1];
            } else if bank[i] > ones_digit {
                ones_digit = bank[i];
            }
        }

        if bank[bank.len() - 1] > ones_digit {
            ones_digit = bank[bank.len() - 1];
        }
        let biggest = tens_digit * 10 + ones_digit;

        result += biggest;
    }
    result
}

pub fn part2(input: &str) -> u64 {
    let mut result = 0;

    for l in input.lines() {
        let bank: Vec<u32> = l.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let mut digits: Vec<u64> = bank
            .iter()
            .cloned()
            .rev()
            .take(12)
            .rev()
            .map(u64::from)
            .collect();

        for i in (0..bank.len() - 12).rev() {
            shift_digits(bank[i], &mut digits);
        }

        let joltage = digits.iter().fold(0, |sum, d| (sum * 10) + *d);
        result += joltage;
    }
    result
}

/// shift_digits(9, [3, 1, 2]) mutates the vec to [9, 3, 2]
fn shift_digits(candidate: u32, digits: &mut [u64]) {
    let mut shifter: u64 = candidate as u64;
    for i in digits {
        if shifter >= *i {
            std::mem::swap(&mut shifter, &mut *i);
        } else {
            break;
        }
    }
}
