use std::cmp::max;
use std::env;
use std::fs;

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

fn is_draw_valid(draw: &str) -> bool {
    let colours: Vec<&str> = draw.split(",").collect();
    for colour in colours {
        let temp: Vec<&str> = colour.split_ascii_whitespace().collect();
        let num = temp[0].parse::<i32>().unwrap();
        let name = temp[1].trim();
        match name {
            "red" => if num > RED {return false}
            "green" => if num > GREEN {return false}
            "blue" => if num > BLUE {return false}
            _ => return false
        }
    }
    true
}


fn part1(input: &Vec<&str>) -> i32 {
    let mut sum: i32 = 0;
    'games: for (i, game) in input.iter().enumerate() {
        let draws: Vec<&str> = (*game).split(":").collect();
        let draws: &str = draws[1].clone();
        let draws: Vec<&str> = draws.trim().split(";").collect();
        for draw in draws {
            if !is_draw_valid(draw.trim()) {
                continue 'games
            }
        }
        sum += i as i32 + 1;
    }
    sum
}

fn part2(input: &Vec<&str>) -> i32 {
    let mut sum: i32 = 0;
    'games: for (i, game) in input.iter().enumerate() {
        let draws: Vec<&str> = (*game).split(":").collect();
        let draws: &str = draws[1].clone();
        let draws: Vec<&str> = draws.trim().split(";").collect();

        let mut max_red: i32 = 0;
        let mut max_green: i32 = 0;
        let mut max_blue: i32 = 0;

        for draw in draws {
            let colours: Vec<&str> = draw.split(",").collect();

            for colour in colours {
                let temp: Vec<&str> = colour.split_ascii_whitespace().collect();
                let num = temp[0].parse::<i32>().unwrap();
                let name = temp[1].trim();

                match name {
                    "red" => max_red = max(max_red, num),
                    "green" => max_green = max(max_green, num),
                    "blue" => max_blue = max(max_blue, num),
                    _ => ()
                }
            }
        }
        sum += max_red * max_blue * max_green;
    }
    sum
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().collect::<Vec<String>>()[1].clone();
    let buf = fs::read_to_string(path)?;
    let input: Vec<&str> = buf.trim().split('\n').collect();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
    Ok(())
}
