use std::collections::{HashSet, VecDeque};
use std::fs;

fn part1_calc_points(card: &str) -> i32 {
    let mut split = card.split(" | ");
    let winning: &str = split.next().unwrap();
    let owned: &str = split.next().unwrap();
    let winning: HashSet<i32> = winning.split_ascii_whitespace().map(|x| x.trim().parse::<i32>().unwrap()).collect();
    let owned: Vec<i32> = owned.split_ascii_whitespace().map(|x| x.trim().parse::<i32>().unwrap()).collect();

    let mut winning_on_card = 0;
    for number in &owned {
        winning_on_card += match winning.contains(number) {
            true => 1,
            false => 0,
        }
    }
    match winning_on_card {
        0 => 0,
        n if n > 0 => 2_i32.pow((n - 1) as u32),
        _ => 0
    }
}

fn part1(input: Vec<&str>) -> i32 {
    let mut sum: i32 = 0;
    for line in &input {
        sum += part1_calc_points((*line).split(": ").nth(1).unwrap());
    }
    sum
}

fn part2_count_winning(card: &str) -> i32 {
    let mut split = card.split(" | ");
    let winning: &str = split.next().unwrap();
    let owned: &str = split.next().unwrap();
    let winning: HashSet<i32> = winning.split_ascii_whitespace().map(|x| x.trim().parse::<i32>().unwrap()).collect();
    let owned: Vec<i32> = owned.split_ascii_whitespace().map(|x| x.trim().parse::<i32>().unwrap()).collect();

    let mut winning_on_card = 0;
    for number in &owned {
        winning_on_card += match winning.contains(number) {
            true => 1,
            false => 0,
        }
    }
    winning_on_card
}

fn process_card(card_id: i32, winning_per_card: &Vec<i32>, processing_queue: &mut VecDeque<i32>) -> i32 {
    let card_winnings: &i32 = winning_per_card.get(card_id as usize).unwrap();
    let mut cards_added: i32 = 0;
    for id in card_id + 1..=card_id + card_winnings  {
        cards_added += 1;
        processing_queue.push_back(id);
    }
    cards_added
}

fn part2(input: Vec<&str>) -> i32 {
    let mut winning_per_card: Vec<i32> = Vec::new();
    winning_per_card.push(-1);
    let mut processing_queue: VecDeque<i32> = VecDeque::new();
    for (i, line) in input.iter().enumerate() {
        winning_per_card.push(part2_count_winning((*line).split(": ").nth(1).unwrap()));
        processing_queue.push_back(i as i32 + 1);
    }

    let mut cards_won: i32 = processing_queue.len() as i32;
    while !processing_queue.is_empty() {
        cards_won += process_card(processing_queue.pop_back().unwrap(), &winning_per_card, &mut processing_queue);
    }

    cards_won
}

fn main() {
    let buf = fs::read_to_string("./input").unwrap();
    let input: Vec<&str> = buf.trim().split('\n').collect();
    println!("Part 1: {}", part1(input.clone()));
    println!("Part 2: {}", part2(input));
}
#[cfg(test)]
mod part1_tests {
    use super::*;

    #[test]
    fn example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let input: Vec<&str> = input.trim().split('\n').collect();
        assert_eq!(part1(input), 13);
    }
}

#[cfg(test)]
mod part2_tests {
    use super::*;

    #[test]
    fn example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let input: Vec<&str> = input.trim().split('\n').collect();
        assert_eq!(part2(input), 30);
    }
}
