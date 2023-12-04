use std::fs;
use std::env;

fn get_num_from_line(line: &str) -> i32 {
    let mut first_dig: Option<char> = None;
    let mut last_dig: Option<char> = None;

    for c in line.chars() {
        if !c.is_numeric() {
            continue
        }

        if let None = first_dig {
            first_dig = Some(c);
        }
        last_dig = Some(c);
    }
    (10 * first_dig.unwrap().to_digit(10).unwrap() + last_dig.unwrap().to_digit(10).unwrap()) as i32
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
