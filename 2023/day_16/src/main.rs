use std::fs;

mod part1 {
    use std::collections::{BinaryHeap, HashSet};

    #[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
    enum Direction {
        Right,
        Left,
        Up,
        Down,
    }

    #[derive(Hash)]
    #[derive(Eq, PartialEq)]
    struct State {
        pos: (i32, i32),
        direction: Direction,
    }

    enum BeamAction {
        Nothing,
        SpawnBeam(Beam),
        Suicide,
    }

    struct Beam {
        pub pos: (i32, i32),
        pub direction: Direction,
        rows: i32,
        columns: i32,
    }

    impl Beam {
        fn new(pos: (i32, i32), direction: Direction, layout: &Vec<Vec<char>>) -> Self {
            Self {
                pos,
                direction,
                rows: layout.len() as i32,
                columns: layout.first()
                    .unwrap().len() as i32,
            }
        }

        fn new_with_concrete_dimensions(pos: (i32, i32), direction: Direction, rows: i32, columns: i32) -> Self {
            Self { pos, direction, rows, columns }
        }

        fn update(&mut self, layout: &Vec<Vec<char>>, states: &mut HashSet<State>) -> BeamAction {
            match self.direction { // update position
                Direction::Right => { self.pos.1 += 1; }
                Direction::Left => { self.pos.1 -= 1; }
                Direction::Down => { self.pos.0 += 1; }
                Direction::Up => { self.pos.0 -= 1; }
            }

            // kill beam if necessary
            if self.pos.0 < 0 || self.pos.1 < 0 ||
                self.pos.0 >= self.rows || self.pos.1 >= self.columns { // position outside layout
                return BeamAction::Suicide;
            }


            // process current tile
            let tile: char = layout[self.pos.0 as usize][self.pos.1 as usize];
            let mut action = BeamAction::Nothing;
            match tile {
                '|' => {
                    match self.direction {
                        Direction::Left | Direction::Right => {
                            self.direction = Direction::Up;
                            action = BeamAction::SpawnBeam(self.opposite_beam());
                        }
                        Direction::Up | Direction::Down => {}
                    }
                }
                '-' => {
                    match self.direction {
                        Direction::Right | Direction::Left => {}
                        Direction::Up | Direction::Down => {
                            self.direction = Direction::Left;
                            action = BeamAction::SpawnBeam(self.opposite_beam());
                        }
                    }
                }
                '\\' => {
                    match self.direction {
                        Direction::Left => { self.direction = Direction::Up; }
                        Direction::Right => { self.direction = Direction::Down; }
                        Direction::Down => { self.direction = Direction::Right; }
                        Direction::Up => { self.direction = Direction::Left; }
                    }
                }
                '/' => {
                    match self.direction {
                        Direction::Left => { self.direction = Direction::Down; }
                        Direction::Right => { self.direction = Direction::Up; }
                        Direction::Down => { self.direction = Direction::Left; }
                        Direction::Up => { self.direction = Direction::Right; }
                    }
                }
                _ => {}
            }

            let state = State { pos: self.pos, direction: self.direction };
            if states.contains(&state) { // previously visited this tile, with same direction - cycle
                return BeamAction::Suicide;
            }
            states.insert(state);

            action
        }

        fn opposite_beam(&self) -> Beam {
            match self.direction {
                Direction::Right => Beam::new_with_concrete_dimensions(self.pos, Direction::Left, self.rows, self.columns),
                Direction::Left => Beam::new_with_concrete_dimensions(self.pos, Direction::Right, self.rows, self.columns),
                Direction::Up => Beam::new_with_concrete_dimensions(self.pos, Direction::Down, self.rows, self.columns),
                Direction::Down => Beam::new_with_concrete_dimensions(self.pos, Direction::Up, self.rows, self.columns),
            }
        }
    }

    pub fn solve(input: &str) -> i32 {
        let layout: Vec<Vec<char>> = input.split("\n")
            .map(|line| line.trim().chars().collect())
            .collect();

        let mut beams: Vec<Beam> = Vec::new();
        beams.push(Beam::new((0, 0), Direction::Right, &layout));

        let mut states = HashSet::new();
        let mut energized: HashSet<(i32, i32)> = HashSet::new();
        energized.insert((0, 0)); // starting tile

        while beams.len() > 0 {
            let mut beams_for_execution: BinaryHeap<usize> = BinaryHeap::new();
            let mut newborn_beams = Vec::new();
            for (i, beam) in beams.iter_mut().enumerate() {
                let action = beam.update(&layout, &mut states);
                match action {
                    BeamAction::Suicide => { beams_for_execution.push(i); continue; }
                    BeamAction::SpawnBeam(baby_beam) => { newborn_beams.push(baby_beam); }
                    _ => {}
                }
                energized.insert(beam.pos);
            }
            // execute
            while let Some(i) = beams_for_execution.pop() { beams.remove(i); }
            // birth new beams
            for baby_beam in newborn_beams { beams.push(baby_beam); }
        }
        energized.len() as i32
    }
}

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    println!("Part 1: {}", part1::solve(&input));
}

#[cfg(test)]
mod pt1_tests {
    use super::*;

    #[test]
    fn example() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(46, part1::solve(input));
    }
}