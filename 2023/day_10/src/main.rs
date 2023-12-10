use std::fs;

mod part1 {
    use std::cmp::min;
    use std::collections::{HashMap, VecDeque};

    pub fn solve(input: &str) -> i32 {
        let input: Vec<Vec<char>> = input.lines()
            .map(|line| {
                line.chars().collect()
            }).collect();
        let mut graph = Graph::new(&input);

        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        q.push_back(graph.start);
        let mut dist: HashMap<(i32, i32), i32> = HashMap::new();
        dist.insert(graph.start, 0);

        while !q.is_empty() {
            let current_node= q.pop_front().unwrap();
            let current_distance = *dist.get(&current_node).unwrap();
            for node in graph.get_connected(&current_node) {
                match dist.get_mut(node) {
                    Some(distance) => { *distance = min(*distance, current_distance + 1); }
                    None => { dist.insert(*node, current_distance + 1); q.push_back(*node); }
                }
            }
        }
        *(dist.iter().max_by(|(_, a), (_, b)| (**a).cmp(*b)).unwrap().1)
    }

    struct Graph {
        nodes: HashMap<(i32, i32), Vec<(i32, i32)>>,
        start: (i32, i32),
    }

    impl Graph {
        fn new(map: &Vec<Vec<char>>) -> Self {
            let mut parsed_map: Graph = Graph { nodes: HashMap::new(), start: (0, 0) };

            for (i, vec) in map.iter().enumerate() {
                for (j, cell) in vec.iter().enumerate() {
                    parsed_map.process_cell(cell, &(i as i32, j as i32))
                }
            }

            let nodes = parsed_map.nodes.clone();
            let vals = nodes.iter();
            let start = parsed_map.start.clone();
            for (node, nodes_connected) in vals {
                for node_connected in nodes_connected {
                    if node_connected == &parsed_map.start {
                        parsed_map.make_connection(&start, node);
                    }
                }
            }

            parsed_map
        }

        fn make_connection(&mut self, source: &(i32, i32), dest: &(i32, i32)) {
            if !self.nodes.contains_key(source) { self.nodes.insert(source.clone(), Vec::new()); }
            match self.nodes.get_mut(source) {
                None => {
                    self.nodes.insert(source.clone(), vec![dest.clone()]);
                }
                Some(node) => { node.push(dest.clone()); }
            }
        }

        fn process_cell(&mut self, cell_symbol: &char, cell_pos: &(i32, i32)) {
            match cell_symbol {
                '|' => {
                    self.make_connection(cell_pos, &(cell_pos.0 - 1, cell_pos.1));
                    self.make_connection(cell_pos, &(cell_pos.0 + 1, cell_pos.1));
                }
                '-' => {
                    self.make_connection(cell_pos, &(cell_pos.0, cell_pos.1 - 1));
                    self.make_connection(cell_pos, &(cell_pos.0, cell_pos.1 + 1));
                }
                'L' => {
                    self.make_connection(cell_pos, &(cell_pos.0 - 1, cell_pos.1));
                    self.make_connection(cell_pos, &(cell_pos.0, cell_pos.1 + 1));
                }
                'J' => {
                    self.make_connection(cell_pos, &(cell_pos.0 - 1, cell_pos.1));
                    self.make_connection(cell_pos, &(cell_pos.0, cell_pos.1 - 1));
                }
                '7' => {
                    self.make_connection(cell_pos, &(cell_pos.0, cell_pos.1 - 1));
                    self.make_connection(cell_pos, &(cell_pos.0 + 1, cell_pos.1));
                }
                'F' => {
                    self.make_connection(cell_pos, &(cell_pos.0, cell_pos.1 + 1));
                    self.make_connection(cell_pos, &(cell_pos.0 + 1, cell_pos.1));
                }
                'S' => { self.start = *cell_pos; }
                _ => {}
            }
        }

        fn get_connected(&mut self, node: &(i32, i32)) -> &Vec<(i32, i32)> {
            match self.nodes.contains_key(node) {
                true => { self.nodes.get(node) }
                false => {
                    self.nodes.insert(*node, Vec::new());
                    self.nodes.get(node)
                }
            }.unwrap()
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    println!("Part 1: {}", part1::solve(&input));
}

#[cfg(test)]
mod part1_test {
    use super::*;

    #[test]
    fn example1() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(4, part1::solve(input));
    }

    #[test]
    fn example2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(8, part1::solve(input));
    }
}
