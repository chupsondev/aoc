use std::fs;

pub mod part1 {
    pub fn solve(input: &str) -> i32 {
        let sequences: Vec<Vec<i32>> = input.trim().split('\n')
            .map(|seq| {
                seq.split_ascii_whitespace()
                    .map(|num| num.trim().parse::<i32>().unwrap())
                    .collect()
            }).collect();

        let mut sum = 0;
        for sequence in &sequences {
            sum += extrapolate_last_num(sequence);
        }
        sum
    }

    fn extrapolate_last_num(seq: &Vec<i32>) -> i32 {
        let mut all_same = true;
        let first_num = &seq[0];
        for num in seq { all_same = all_same && num == first_num; }
        if all_same { return first_num.clone(); }

        let mut differences_sequence: Vec<i32> = Vec::new();
        for (i, v) in seq[1..].iter().enumerate() {
            differences_sequence.push(v - seq[i]);
        }
        seq.last().unwrap() + extrapolate_last_num(&differences_sequence)
    }
}

pub mod part2 {
    pub fn solve(input: &str) -> i32 {
        let sequences: Vec<Vec<i32>> = input.trim().split('\n')
            .map(|seq| {
                seq.split_ascii_whitespace()
                    .map(|num| num.trim().parse::<i32>().unwrap())
                    .collect()
            }).collect();

        let mut sum = 0;
        for sequence in &sequences {
            sum += extrapolate_first_num(sequence);
        }
        sum
    }

    fn extrapolate_first_num(seq: &Vec<i32>) -> i32 {
        let mut all_same = true;
        let first_num = &seq[0];
        for num in seq { all_same = all_same && num == first_num; }
        if all_same { return first_num.clone(); }

        let mut differences_sequence: Vec<i32> = Vec::new();
        for (i, v) in seq[1..].iter().enumerate() {
            differences_sequence.push(v - seq[i]);
        }
        seq.first().unwrap() - extrapolate_first_num(&differences_sequence)
    }
}

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    println!("Part 1: {}", part1::solve(&input));
    println!("Part 2: {}", part2::solve(&input));
}

#[cfg(test)]
mod test_part1 {
    use super::*;
    #[test]
    fn example() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(114, part1::solve(&input));
    }
}

#[cfg(test)]
mod test_part2 {
    use super::*;
    #[test]
    fn example() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(2, part2::solve(&input));
    }
}
