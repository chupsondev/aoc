use std::fs;

mod part1 {
    pub fn solve(input: &str) -> i32 {
        let step_hashes = input.split(',').map(|step| {
            let step = step.trim();
            hash(step)
        });

        step_hashes.sum()
    }

    fn hash(string:  &str) -> i32 {
        let mut cur_val = 0;
        for c in string.chars() {
            cur_val += c as i32;
            cur_val *= 17;
            cur_val = cur_val % 256;
        }
        cur_val
    }
}

mod part2 {
    use std::collections::HashMap;

    struct Lens {
        label: String,
        pub focal_length: i32
    }

    impl Lens {
        fn new(label: &str, focal_length: i32) -> Self {
            Lens { label: String::from(label), focal_length }
        }

        fn vec_contains_same_label(v: &Vec<Self>, label: &str) -> Option<usize> {
            for (i, lens) in v.iter().enumerate() {
                if lens.label == label {
                    return Some(i);
                }
            }
            None
        }
    }

    pub fn solve(input: &str) -> i32 {
        let mut boxes: HashMap<i32, Vec<Lens>> = HashMap::new();

        let steps = input.split(',').map(|step| {
            let step = step.trim();
            step
        });

        for step in steps {
            if step.ends_with('-') {
                let label = step.split('-').next().unwrap();
                let label_hash = hash(label);


                if !boxes.contains_key(&label_hash) { boxes.insert(label_hash, Vec::new()); }
                let v = boxes.get_mut(&label_hash).unwrap();
                if let Some(i) = Lens::vec_contains_same_label(v, label) {
                    v.remove(i);
                }
                continue;
            }

            let mut step = step.split("=");
            let label = step.next().unwrap();
            let label_hash = hash(label);
            let focal_length: i32 = step.next().unwrap().parse().unwrap();

            if !boxes.contains_key(&label_hash) { boxes.insert(label_hash, Vec::new()); }
            let v = boxes.get_mut(&label_hash).unwrap();
            match Lens::vec_contains_same_label(v, label) {
                None => { v.push(Lens::new(label, focal_length)) }
                Some(i) => v.get_mut(i).unwrap().focal_length = focal_length
            }

        }

        let mut sum = 0;
        for i in 0..256 {
            if !boxes.contains_key(&i) { continue; }
            for (j, lens) in boxes.get(&i).unwrap().iter().enumerate() {
                sum += (i + 1) * (j as i32 + 1) * lens.focal_length;
            }
        }

        sum
    }

    fn hash(string:  &str) -> i32 {
        let mut cur_val = 0;
        for c in string.chars() {
            cur_val += c as i32;
            cur_val *= 17;
            cur_val = cur_val % 256;
        }
        cur_val
    }
}

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    println!("Part 1: {}", part1::solve(&input));
    println!("Part 2: {}", part2::solve(&input));
}

#[cfg(test)]
mod pt1_test {
    use super::*;

    #[test]
    fn example() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(1320, part1::solve(input));
    }
}
#[cfg(test)]
mod pt2_test {
    use super::*;

    #[test]
    fn example() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(145, part2::solve(input));
    }
}
