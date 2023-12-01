use std::env;
use std::fs;

const SPELLED_NUMS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

enum SpelledNum {
    None,
    Beginning,
    Some(i32)
}

fn is_spelled_num(spelling: &str) -> SpelledNum {
    for (i, full_spelling) in SPELLED_NUMS.iter().enumerate() {
        let full_spelling = full_spelling.clone();
        if full_spelling == spelling {
            return SpelledNum::Some((i + 1) as i32)
        }
        if full_spelling.starts_with(spelling) {
            return SpelledNum::Beginning;
        }
    }
    SpelledNum::None
}

fn check_part(part: &str) -> Option<i32> {
    if part.chars().next().unwrap().is_numeric() {
        return Some(part.chars().next().unwrap().to_digit(10).unwrap() as i32);
    }
    let mut current_spelled = String::new();
    for c in part.chars() {
        current_spelled.push(c);
        let spelling = is_spelled_num(&current_spelled);
        if let SpelledNum::None = spelling {
            return None;
        }
        if let SpelledNum::Some(n) = spelling {
            return Some(n);
        }
    }
    None
}

fn get_num_from_line(line: &str) -> i32 {
    let mut first_dig: Option<i32> = None;
    let mut last_dig: Option<i32> = None;

    let mut current_spelled: String = String::new();
    let mut i = 0;
    for c in line.chars() {
        let mut digit: Option<i32> = check_part(&line[i..]);
        // if c.is_numeric() {
        //     digit = Some(c.to_digit(10).unwrap() as i32);
        //     current_spelled.clear();
        // } else {
        //     current_spelled.push(c);
        //     let spelled_num = is_spelled_num(&current_spelled);
        //     if let SpelledNum::None = spelled_num {
        //         current_spelled = String::from(c);
        //     }
        //     if let SpelledNum::Some(n) = spelled_num {
        //         digit = Some(n);
        //         current_spelled = String::from(c);
        //     }
        // }

        i += 1;
        if let None = digit { continue }

        if let None = first_dig {
            first_dig = digit;
        }
        last_dig = digit;

    }
    10 * first_dig.unwrap() + last_dig.unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();

    let binding = fs::read_to_string(file_path)?;
    let lines: Vec<&str> = binding.trim().split('\n').collect();
    let mut sum = 0;
    for line in lines {
        sum += get_num_from_line(line);
    }
    println!("{}", sum);
    Ok(())
}
