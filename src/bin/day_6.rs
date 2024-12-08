use aoc24::{get_input, parse_input_matrix};
use std::collections::HashSet;

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    pos_guard: (i32, i32),
    direction: (i32, i32),
    visited: HashSet<(i32, i32)>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut map = parse_input_matrix(input.parse().unwrap());
        let position = map
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, &ch)| {
                    if ch == '^' {
                        Some((x as i32, y as i32))
                    } else {
                        None
                    }
                })
            })
            .unwrap();
        map[position.1 as usize][position.0 as usize] = 'X';
        Self {
            map,
            pos_guard: position,
            direction: (0, -1),
            visited: Default::default(),
        }
    }

    fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && y < self.map.len() as i32 && x < self.map[y as usize].len() as i32
    }

    fn tick(&mut self) -> bool {
        let next_position = (
            self.pos_guard.0 + self.direction.0,
            self.pos_guard.1 + self.direction.1,
        );

        if !self.is_in_bounds(next_position.0, next_position.1) {
            return false;
        }

        let char_at_next_position = self.map[next_position.1 as usize][next_position.0 as usize];
        if char_at_next_position == '#' {
            self.turn_clockwise();
        } else {
            self.pos_guard = next_position;
            self.map[self.pos_guard.1 as usize][self.pos_guard.0 as usize] = 'X';
            self.visited.insert(self.pos_guard);
        }

        true
    }

    fn turn_clockwise(&mut self) {
        self.direction = match self.direction {
            (0, -1) => (1, 0),
            (1, 0) => (0, 1),
            (0, 1) => (-1, 0),
            (-1, 0) => (0, -1),
            _ => panic!("invalid direction"),
        };
    }

    fn simulate_obstruction(&mut self, x: i32, y: i32) -> bool {
        if self.pos_guard == (x, y) {
            return false;
        }

        let original_char = self.map[y as usize][x as usize];

        self.map[y as usize][x as usize] = '#';

        let is_loop = self.is_guard_in_loop();

        self.map[y as usize][x as usize] = original_char;

        is_loop
    }

    fn is_guard_in_loop(&mut self) -> bool {
        let original_pos_guard = self.pos_guard;
        let original_direction = self.direction;
        let mut visited_locations: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

        while self.tick() {
            if visited_locations.contains(&(self.pos_guard, self.direction)) {
                self.pos_guard = original_pos_guard;
                self.direction = original_direction;
                return true;
            }
            visited_locations.insert((self.pos_guard, self.direction));
        }

        self.pos_guard = original_pos_guard;
        self.direction = original_direction;

        false
    }
}

fn main() {
    let input = get_input(6, false);
    let mut sum2 = 0;

    let mut map = Map::new(&input);

    while map.tick() {}

    let sum1: i32 = map
        .map
        .iter()
        .map(|row| row.iter().filter(|&&ch| ch == 'X').count() as i32)
        .sum();

    let visited = map.visited.clone();

    map = Map::new(&input);
    for (x, y) in visited {
        sum2 += if map.simulate_obstruction(x, y) { 1 } else { 0 };
    }

    println!("sum1: {}", sum1);
    println!("sum2: {}", sum2);
}
