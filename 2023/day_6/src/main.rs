use std::fs;

struct Race {
    time: i32,
    record: i32
}

impl Race {
    fn new(time: i32, record: i32) -> Self {
        Race{time, record}
    }
    fn count_win_methods(&self) -> i32 {
        let mut methods = 0;
        for i in 0..=self.time {
            if (self.time - i) * i > self.record { methods += 1; }
        }
        methods
    }
}

struct Race2 {
    time: u64,
    record: u64
}

impl Race2 {
    fn new(time: u64, record: u64) -> Self {
        Race2 {time, record}
    }
    fn count_win_methods(&self) -> u64 {
        let mut i = 0;
        for _ in 0..=self.time {
            if (self.time - i) * i > self.record { break; }
            i += 1;
        }
        match self.time % 2 {
            0 => (self.time / 2 - i) * 2 + 1,
            1 => ((self.time + 1) / 2 - i) * 2,
            _ => 0
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut input = input.split('\n');
    let time: Vec<i32> = input.next().unwrap()
        .split(":").last().unwrap()
        .split_ascii_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap_or_else(|f| {println!("{}", f); 0}))
        .collect();
    let record: Vec<i32> = input.next().unwrap()
        .split(":").last().unwrap()
        .split_ascii_whitespace()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    let mut races: Vec<Race> = Vec::new();

    for (i, v) in time.iter().enumerate() {
        races.push(Race::new(*v, *record.get(i).unwrap()))
    }

    let mut res = 1;
    for race in &races {
        res *= race.count_win_methods();
    }

    res
}

fn part2(input: &str) -> u64 {
    let mut input = input.split('\n');
    let time: u64 = input.next().unwrap()
        .split(":").last().unwrap()
        .split_ascii_whitespace().collect::<String>().parse().unwrap();
    let record: u64 = input.next().unwrap()
        .split(":").last().unwrap()
        .split_ascii_whitespace().collect::<String>().parse().unwrap();

    let mut races: Vec<Race2> = Vec::new();

    races.push(Race2::new(time, record));

    let mut res = 1;
    for race in &races {
        res *= race.count_win_methods();
    }

    res
}
fn main() {
    let input = fs::read_to_string("./input").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod part1 {
    use crate::part1;

    #[test]
    fn example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(288, part1(input));
    }
}

#[cfg(test)]
mod part2 {
    use crate::part2;

    #[test]
    fn example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(71503, part2(input));
    }
}
