use std::collections::HashMap;
use std::fs;

enum Directions {
    Left,
    Right
}

fn part1(input: &str) -> i32 {
    let mut input = input.split("\n\n");
    let directions = input.next().unwrap();
    let directions: Vec<Directions> = directions.trim().chars().map(|c| match c {
        'L' => Directions::Left,
        'R' => Directions::Right,
        _ => Directions::Right
    }).collect();

    let nodes_string = input.next().unwrap();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in nodes_string.trim().lines() {
        let mut parted = line.split(" = ");
        let node = parted.next().unwrap();
        let destinations = parted.next().unwrap();
        let destinations = &destinations[1..destinations.len() - 1];
        let destinations = destinations.split(", ").map(|x|
            x.trim()).collect::<Vec<&str>>();

        map.insert(node, (destinations[0], destinations[1]));
    }

    let mut current = "AAA";
    let mut steps = 0;
    while current != "ZZZ" {
        let direction = &directions[steps % directions.len()];
        match direction {
            Directions::Left => { current = map.get(current).unwrap().0; }
            Directions::Right => { current = map.get(current).unwrap().1; }
        }
        steps += 1;
    }
    steps as i32
}

fn all_arrived(nodes: &Vec<&str>) -> bool {
    for node in nodes {
        if !node.ends_with('Z') { return false }
    }
    true
}
fn part2(input: &str) -> i32 {
    let mut input = input.split("\n\n");
    let directions = input.next().unwrap();
    let directions: Vec<Directions> = directions.trim().chars().map(|c| match c {
        'L' => Directions::Left,
        'R' => Directions::Right,
        _ => Directions::Right
    }).collect();

    let nodes_string = input.next().unwrap();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut starting: Vec<&str> = Vec::new();
    for line in nodes_string.trim().lines() {
        let mut parted = line.split(" = ");
        let node = parted.next().unwrap();
        if node.ends_with('A') { starting.push(node); }

        let destinations = parted.next().unwrap();
        let destinations = &destinations[1..destinations.len() - 1];
        let destinations = destinations.split(", ").map(|x|
            x.trim()).collect::<Vec<&str>>();

        map.insert(node, (destinations[0], destinations[1]));
    }

    let mut current = starting;
    let mut steps = 0;
    let mut all_arrived: bool = false;
    println!("{}", current.len());
    while !all_arrived {
        let direction = &directions[steps % directions.len()];
        all_arrived = true;
        for node in &mut current {
            match direction {
                Directions::Left => { *node = map.get(node).unwrap().0; }
                Directions::Right => { *node = map.get(node).unwrap().1; }
            }
            all_arrived = all_arrived && node.ends_with('Z');
        }
        steps += 1;
    }
    steps as i32
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
    fn example () {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(2, part1(input));
    }
    #[test]
    fn example2 () {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(6, part1(input));
    }
}
#[cfg(test)]
mod part2 {
    use super::*;

    #[test]
    fn example () {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(6, part2(input));
    }
}
