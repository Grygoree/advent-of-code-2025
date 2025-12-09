fn main() {
    let input = include_str!("../input.txt").trim();

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn is_invalid(num: u64) -> bool {
    let digits = num.ilog10() + 1;

    if digits % 2 == 1 {
        return false;
    }

    let half_digits = digits / 2;
    let magnitude = 10_u64.pow(half_digits);

    let left = num / magnitude;
    let right = num % magnitude;

    left == right
}

pub fn part1(input: &str) -> u64 {
    let number_pairs: Vec<(u64, u64)> = input
        .split(",")
        .map(|s| {
            println!("{s}");
            let (left, right) = s.split_once("-").unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect();
    let mut sum_of_invalid_nums = 0;

    for pair in number_pairs {
        println!("{:?}", pair);
        for x in pair.0..=pair.1 {
            if is_invalid(x) {
                sum_of_invalid_nums += x
            }
        }
    }
    sum_of_invalid_nums
}

fn is_invalid_thrice(num: u64) -> bool {
    let digits = num.ilog10() + 1;

    if digits % 3 != 0 {
        return false;
    }

    let third_digits = digits / 3;
    let magnitude = 10_u64.pow(third_digits);

    let left = num / (magnitude * magnitude);
    let right = num % magnitude;
    let middle = (num / magnitude) % magnitude;

    print!("{},{},{}", left, middle, right);
    left == right && middle == left
}

fn is_invalid_five_times(num: u64) -> bool {
    let digits = num.ilog10() + 1;

    if digits % 5 != 0 {
        return false;
    }

    let fifth_digits = digits / 5;
    let mag = 10_u64.pow(fifth_digits);

    let mut seqs = vec![];

    for i in 0..5 {
        seqs.push({
            let selection = mag.pow(i);
            (num / selection) % mag
        });
    }

    let first = seqs[0];

    for seq in seqs {
        if seq != first {
            return false;
        }
    }

    true
}

fn is_invalid_seven_times() {}

fn juan_method(num: u64) {
    let prime_candidates = [2, 3, 5, 7];
    let digits = num.ilog10() + 1;
    let sequence_lengths = prime_candidates
        .iter()
        .copied()
        .filter(|p| digits % p == 0)
        .map(|p| p + 1);

    todo!("Bored");
}

pub fn part2(_input: &str) -> i64 {
    // TODO
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ninety_nine_is_invalid() {
        assert!(is_invalid(99));
    }

    #[test]
    fn one_hundred_eleven_is_invalid_thrice() {
        assert!(is_invalid_thrice(111));
    }

    #[test]
    fn this_num_212121_is_invalid_thrice() {
        assert!(is_invalid_thrice(212121));
    }

    #[test]
    fn invalid_five_times() {}
}
