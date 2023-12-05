use std::cmp::min;
use std::fs;

struct Mapping {
    source: u64,
    dest: u64,
    range: u64
}
impl Mapping {
    fn check_mapping(&self, val: u64) -> Option<u64> {
        if !(val >= self.source && val < self.source + self.range) {
            return None;
        }
        Some(self.dest + (val - self.source))
    }
}

struct FarmMap {
    mappings: Vec<Mapping>
}

impl FarmMap {
    fn new() -> Self {
        FarmMap{mappings: Vec::new()}
    }
    fn add_mapping(&mut self, mapping: Mapping) {
        self.mappings.push(mapping);
    }
    fn map(&self, value: u64) -> u64 {
        for mapping in &self.mappings {
            if let Some(n) = mapping.check_mapping(value) {
                return n;
            }
        }
        value
    }
}

fn part1(input: &str) -> u64 {
    let sections: Vec<&str> = input.trim().split("\n\n").collect();
    let seeds: Vec<u64> = sections.first().unwrap().split(": ").last().unwrap()
        .split(" ")
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();
    let sections = &sections[1..];
    let mut maps: Vec<FarmMap> = Vec::new();
    for section in sections {
        let section = section.split(":\n").last().unwrap();
        maps.push(FarmMap::new());
        let map: &mut FarmMap = maps.last_mut().unwrap();
        for mapping in section.split("\n") {
            let v: Vec<u64> = mapping.split_ascii_whitespace()
                .map(|x| x.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            let dest = v[0];
            let source = v[1];
            let range = v[2];
            map.add_mapping(Mapping{source, dest, range});
        }
    }

    let mut min_location = u64::MAX;
    for seed in seeds {
        let mut value = seed;
        for map in &maps {
            value = map.map(value);
        }
        min_location = min(value, min_location);
    }

    min_location
}

fn part2(input: &str) -> u64 {
    let sections: Vec<&str> = input.trim().split("\n\n").collect();
    let seed_numbers: Vec<u64> = sections.first().unwrap().split(": ").last().unwrap()
        .split(" ")
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();

    let mut seeds: Vec<(u64, u64)> = Vec::new();
    for (i, v) in seed_numbers.iter().enumerate().step_by(2) {
        seeds.push((*v, seed_numbers[i + 1]))
    }

    let sections = &sections[1..];
    let mut maps: Vec<FarmMap> = Vec::new();
    for section in sections {
        let section = section.split(":\n").last().unwrap();
        maps.push(FarmMap::new());
        let map: &mut FarmMap = maps.last_mut().unwrap();
        for mapping in section.split("\n") {
            let v: Vec<u64> = mapping.split_ascii_whitespace()
                .map(|x| x.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            let dest = v[0];
            let source = v[1];
            let range = v[2];
            map.add_mapping(Mapping{source, dest, range});
        }
    }

    let mut min_location = u64::MAX;
    let mut i = 1;
    let total = seeds.len();
    for seed_range in seeds {
        println!("At seed range {i} out of {total}");
        i += 1;
        for seed in seed_range.0..seed_range.0 + seed_range.1 {
            let mut value = seed;
            for map in &maps {
                value = map.map(value);
            }
            min_location = min(value, min_location);
        }
    }

    min_location
}

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn example() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(35, part1(input));
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn example() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(46, part2(input));
    }
}
