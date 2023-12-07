use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;


struct Hand {
    cards: Vec<char>,
    winnings: u64
}

impl Hand {
    fn new(hand: &str) -> Self {
        let mut hand = hand.split_ascii_whitespace();
        let cards = hand.next().unwrap().chars().collect::<Vec<char>>();
        let winnings = hand.next().unwrap().parse::<u64>().unwrap();
        Self {cards, winnings}
    }

    fn get_hand_type_rank(&self) -> u64 {
        let mut counts: HashMap<char, u64> = HashMap::new();
        for card in &self.cards {
            if counts.contains_key(card) {
                *counts.get_mut(card).unwrap() += 1;
            } else {
                counts.insert(*card, 1);
            }
        }

        let mut num_fives = 0;
        let mut num_fours = 0;
        let mut num_threes = 0;
        let mut num_pairs = 0;

        for (_, v) in &counts {
            match v {
                5 => { num_fives += 1; }
                4 => { num_fours += 1; }
                3 => { num_threes += 1; }
                2 => {num_pairs += 1; }
                _ => {}
            }
        }

        if num_fives > 0 { return 6; }
        if num_fours > 0 {return 5; }
        if num_threes == 1 && num_pairs == 1 { return 4; }
        if num_threes == 1 { return 3; }
        if num_pairs == 2 { return 2; }
        if num_pairs == 1 { return 1; }
        0

    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.winnings == other.winnings
    }
}
impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let card_vals: HashMap<char, u64> = HashMap::from([('2', 0), ('3', 1), ('4', 2), ('5', 3), ('6', 4), ('7', 5), ('8', 6),
            ('9', 7), ('T', 8), ('J', 9), ('Q', 10), ('K', 11), ('A', 12)]);

        match self.get_hand_type_rank().cmp(&other.get_hand_type_rank()) {
            Ordering::Less => { return Ordering::Less; }
            Ordering::Equal => {}
            Ordering::Greater => {return Ordering::Greater; }
        }

        for i in 0..5 {
            let i = i as usize;
            match card_vals.get(&self.cards[i]).unwrap().cmp(card_vals.get(&other.cards[i]).unwrap()) {
                Ordering::Less => { return Ordering::Less; }
                Ordering::Equal => {}
                Ordering::Greater => { return  Ordering::Greater; }
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &str) -> u64 {
    let input: Vec<&str> = input.trim().split('\n').collect();
    let mut hands: Vec<Hand> = Vec::new();
    for line in input {
        hands.push(Hand::new(line));
    }

    hands.sort();
    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += (i as u64 + 1) * hand.winnings;
    }

    sum
}
#[derive(Debug)]
struct Hand2 {
    cards: Vec<char>,
    winnings: u64
}

impl Hand2 {
    fn new(hand: &str) -> Self {
        let mut hand = hand.split_ascii_whitespace();
        let cards = hand.next().unwrap().chars().collect::<Vec<char>>();
        let winnings = hand.next().unwrap().parse::<u64>().unwrap();
        Self {cards, winnings}
    }

    fn get_hand_type_rank(&self) -> u64 {
        let mut counts: HashMap<char, u64> = HashMap::new();
        let mut jokers = 0;
        for card in &self.cards {
            if *card == 'J' {
                jokers += 1;
                continue;
            }
            if counts.contains_key(card) {
                *counts.get_mut(card).unwrap() += 1;
            } else {
                counts.insert(*card, 1);
            }
        }

        let max_key = match counts.iter().max_by(|a, b| a.1.cmp(b.1)) {
            None => {counts.insert('A', 0); 'A'}
            Some(v) => { v.0.clone() }
        };
        *counts.get_mut(&max_key).unwrap() += jokers;

        let mut num_fives = 0;
        let mut num_fours = 0;
        let mut num_threes = 0;
        let mut num_pairs = 0;

        for (_, v) in &counts {
            match v {
                5 => { num_fives += 1; }
                4 => { num_fours += 1; }
                3 => { num_threes += 1; }
                2 => {num_pairs += 1; }
                _ => {}
            }
        }

        if num_fives > 0 { return 6; }
        if num_fours > 0 {return 5; }
        if num_threes == 1 && num_pairs == 1 { return 4; }
        if num_threes == 1 { return 3; }
        if num_pairs == 2 { return 2; }
        if num_pairs == 1 { return 1; }
        0

    }
}

impl PartialEq for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.winnings == other.winnings
    }
}
impl Eq for Hand2 {}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        let card_vals: HashMap<char, u64> = HashMap::from([('2', 1), ('3', 2), ('4', 3), ('5', 4), ('6', 5), ('7', 6), ('8', 7),
            ('9', 8), ('T', 9), ('J', 0), ('Q', 10), ('K', 11), ('A', 12)]);

        match self.get_hand_type_rank().cmp(&other.get_hand_type_rank()) {
            Ordering::Less => { return Ordering::Less; }
            Ordering::Equal => {}
            Ordering::Greater => {return Ordering::Greater; }
        }

        for i in 0..5 {
            let i = i as usize;
            match card_vals.get(&self.cards[i]).unwrap().cmp(card_vals.get(&other.cards[i]).unwrap()) {
                Ordering::Less => { return Ordering::Less; }
                Ordering::Equal => {}
                Ordering::Greater => { return  Ordering::Greater; }
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part2(input: &str) -> u64 {
    let input: Vec<&str> = input.trim().split('\n').collect();
    let mut hands: Vec<Hand2> = Vec::new();
    for line in input {
        hands.push(Hand2::new(line));
    }

    hands.sort();
    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += (i as u64 + 1) * hand.winnings;
    }

    sum
}


fn main() {
    let input = fs::read_to_string("./input").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod part1 {
    use super::*;
    #[test]
    fn example() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(6440, part1(input));
    }
}
#[cfg(test)]
mod part2 {
    use super::*;
    #[test]
    fn example() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(5905, part2(input));
    }
    #[test]
    fn test1() {
        let input = "AAAAA 20
        JJJJJ 10";
        assert_eq!(50, part2(input));
    }
    #[test]
    fn test2() {
        let input = "555JJ 10
        JJ555 5";
        assert_eq!(25, part2(input));
    }
    #[test]
    fn high_card() {
        let hand = Hand2::new("23456 10");
        assert_eq!(0, hand.get_hand_type_rank());
    }
    #[test]
    fn reddit_test() {
        let input = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";
        assert_eq!(6839, part2(input));
    }
    #[test]
    fn testx() {
        let hand: Hand2 = Hand2::new("T55J5 1");
        assert_eq!(5, hand.get_hand_type_rank());
    }
}
