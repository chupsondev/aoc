use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

fn is_adjacent(arr: &Vec<&str>, x: usize, y: usize) -> bool {
    if y > 0 && is_symbol(arr[y - 1].chars().nth(x).unwrap()) { return true; }
    if y < arr.len() - 1 && is_symbol(arr[y + 1].chars().nth(x).unwrap()) { return true; }

    if x > 0 && is_symbol(arr[y].chars().nth(x - 1).unwrap()) { return true; }
    if x < arr[y].len() - 1 && is_symbol(arr[y].chars().nth(x + 1).unwrap())  { return true; }

    if y > 0 && x > 0 && is_symbol(arr[y - 1].chars().nth(x - 1).unwrap()) { return true; }
    if y > 0 && x < arr[y].len() - 1 && is_symbol(arr[y - 1].chars().nth(x + 1).unwrap()) { return true; }
    if y < arr.len() - 1 && x > 0 && is_symbol(arr[y + 1].chars().nth(x - 1).unwrap()) { return true; }
    if y < arr.len() - 1 && x < arr[y].len() - 1 && is_symbol(arr[y + 1].chars().nth(x + 1).unwrap()) { return true; }
    false
}

fn part1(input: &Vec<&str>) -> i32 {
    let mut sum = 0;
    for (y, line) in input.iter().enumerate() {
        let mut number = String::new();
        let mut is_adjacent_: bool = false;
        for (x, c) in (*line).chars().enumerate() {
            if !c.is_numeric() {
                if number.len() == 0 { continue; }
                if !is_adjacent_ { number.clear(); continue; }

                sum += number.parse::<i32>().unwrap();
                number.clear();
                is_adjacent_ = false;
                continue;
            }

            number.push(c);
            is_adjacent_ = is_adjacent(input, x, y) | is_adjacent_;

        }
        if is_adjacent_ && number.len() > 0 { sum += number.parse::<i32>().unwrap(); }
    }
    sum
}

fn is_gear(c: char) -> bool {
    c == '*'
}

fn is_adjacent_to_gear(arr: &Vec<&str>, x: usize, y: usize) -> Vec<(i32, i32)> {
    let xi = x as i32;
    let yi = y as i32;
    let mut v: Vec<(i32, i32)> = Vec::new();
    if y > 0 && is_gear(arr[y - 1].chars().nth(x).unwrap()) { v.push((yi - 1, xi)); }
    if y < arr.len() - 1 && is_gear(arr[y + 1].chars().nth(x).unwrap()) { v.push((yi + 1, xi)); }

    if x > 0 && is_gear(arr[y].chars().nth(x - 1).unwrap()) { v.push((yi, xi - 1)); }
    if x < arr[y].len() - 1 && is_gear(arr[y].chars().nth(x + 1).unwrap())  { v.push((yi, xi + 1)); }

    if y > 0 && x > 0 && is_gear(arr[y - 1].chars().nth(x - 1).unwrap()) { v.push((yi - 1, xi - 1)); }
    if y > 0 && x < arr[y].len() - 1 && is_gear(arr[y - 1].chars().nth(x + 1).unwrap()) { v.push((yi - 1, xi + 1)); }
    if y < arr.len() - 1 && x > 0 && is_gear(arr[y + 1].chars().nth(x - 1).unwrap()) { v.push((yi + 1, xi - 1)); }
    if y < arr.len() - 1 && x < arr[y].len() - 1 && is_gear(arr[y + 1].chars().nth(x + 1).unwrap()) { v.push((yi + 1, xi + 1)); }
    v
}

fn part2(input: &Vec<&str>) -> i32 {
    let mut map: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    for (y, line) in input.iter().enumerate() {
        let mut number = String::new();
        let mut is_adjacent_ = Vec::new();
        for (x, c) in (*line).chars().enumerate() {
            if !c.is_numeric() {
                if number.len() == 0 { continue; }
                if !is_adjacent_.len() == 0 { number.clear(); continue; }

                for gear in is_adjacent_.iter().unique() {
                    match map.get(gear) {
                        None => { map.insert(*gear, vec![number.parse::<i32>().unwrap()]); }
                        Some(_) => { map.get_mut(gear).unwrap().push(number.parse::<i32>().unwrap()); }
                    }
                }
                number.clear();
                is_adjacent_.clear();
                continue;
            }

            number.push(c);
            let mut adjacent_gears = is_adjacent_to_gear(input, x, y);
            is_adjacent_.append(&mut adjacent_gears);

        }
        if is_adjacent_.len() > 0 && number.len() > 0 {
            for gear in is_adjacent_.iter().unique() {
                match map.get(gear) {
                    None => { map.insert(*gear, vec![number.parse::<i32>().unwrap()]); }
                    Some(_) => { map.get_mut(gear).unwrap().push(number.parse::<i32>().unwrap()); }
                }
            }
        }
    }
    let mut sum = 0;
    for (_, v)in map.iter().filter(|t| t.1.len() == 2) {
        sum += v[0] * v[1];
    }
    sum
}

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    let input: Vec<&str> = input.trim().split('\n').collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests_part1 {
    use std::fs;
    use super::*;

    #[test]
    fn part1_example() {
        let input = fs::read_to_string("./part1_example").unwrap();
        let input: Vec<&str> = input.trim().split('\n').collect();
        assert_eq!(part1(&input), 4361);
    }
    #[test]
    fn part1_test1() {
        let input = ".....
....*
..12.".to_string();
        let input: Vec<&str> = input.trim().split('\n').collect();
        assert_eq!(part1(&input), 12);
    }
    #[test]
    fn part1_test2() {
        let input = ".....
.*...
..12.".to_string();
        let input: Vec<&str> = input.trim().split('\n').collect();
        assert_eq!(part1(&input), 12);
    }
    #[test]
    fn part1_test3() {
        let input = ".....
.....
..1-.".to_string();
        let input: Vec<&str> = input.trim().split('\n').collect();
        assert_eq!(part1(&input), 1);
    }
    #[test]
    fn part1_test4() {
        let input = ".....
1.1.1
..1.a
..a..".to_string();
        let input: Vec<&str> = input.trim().split('\n').collect();
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_is_symbol() {
        assert_eq!(is_symbol('.'), false);
        assert_eq!(is_symbol('&'), true);
        assert_eq!(is_symbol('3'), false);
    }
}
#[cfg(test)]
mod tests_part2 {
    use std::fs;
    use super::*;

    #[test]
    fn part2_example() {
        let input = fs::read_to_string("./part1_example").unwrap();
        let input: Vec<&str> = input.trim().split('\n').collect();
        assert_eq!(part2(&input), 467835);
    }
}
