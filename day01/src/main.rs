use std::ops::Neg;

fn main() {
    let input = include_str!("../part1.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

pub fn part1(input: &str) -> i64 {
    let (_sum, zero_count) = input
        .lines()
        .map(|l| match l.split_at(1) {
            ("R", magnitude) => magnitude.parse::<i64>().unwrap(),
            ("L", magnitude) => magnitude.parse::<i64>().unwrap().neg(),
            (_, _) => panic!("bad input"),
        })
        .fold((50, 0), |(sum, zero_count), n| {
            let new_sum = (sum + n) % 100;
            (new_sum, zero_count + (if new_sum == 0 { 1 } else { 0 }))
        });

    zero_count
}

pub fn part2(input: &str) -> i64 {
    let (_sum, zero_count) = input
        .lines()
        .map(|l| match l.split_at(1) {
            ("R", magnitude) => magnitude.parse::<i64>().unwrap(),
            ("L", magnitude) => magnitude.parse::<i64>().unwrap().neg(),
            (_, _) => panic!("bad input"),
        })
        .fold((50, 0), |(position, zero_count), n| {
            let new_sum = position + n;
            let new_position = new_sum.rem_euclid(100);
            let zeros_passed =
                (new_sum / (100)).abs() + if new_sum <= 0 && position != 0 { 1 } else { 0 };
            // println!(
            //     "{:<6} {:<6} {:<6} {:<6}",
            //     new_position, new_sum, zeros_passed, n
            // );
            (new_position, zero_count + zeros_passed)
        });

    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_left_to_zero() {
        let input = "L50";
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn move_right_to_zero() {
        let input = "R50";
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn move_left_past_zero() {
        let input = "L100";
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn move_right_past_zero() {
        let input = "R100";
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn move_right_from_zero_not_passing() {
        let input = "R50\nR50";
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn move_left_from_zero_not_past_zero() {
        let input = "R50\nL50";
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn move_right_not_passing_zero() {
        let input = "R1";
        assert_eq!(part2(input), 0);

        let input = "R49";
        assert_eq!(part2(input), 0);
    }

    #[test]
    fn move_left_not_passing_zero() {
        let input = "L1";
        assert_eq!(part2(input), 0);

        let input = "L49";
        assert_eq!(part2(input), 0);
    }
}
