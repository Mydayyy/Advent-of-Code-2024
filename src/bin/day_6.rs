use aoc24::{get_input, parse_input_matrix};
use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    pos_guard: (i32, i32),
    direction: (i32, i32),
}

impl Map {
    fn new(input: String) -> Self {
        let mut map = parse_input_matrix(input);
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
        println!("{}", map[position.1 as usize][position.0 as usize]);
        map[position.1 as usize][position.0 as usize] = 'X';
        Self {
            map,
            pos_guard: position,
            direction: (0, -1),
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
}

fn main() {
    let input = get_input(6, false);
    let sum2 = 0;
    // println!("map: \n{}", input);

    let mut map = Map::new(input);

    while map.tick() {
        // println!("map: \n{:?}", map);
    }

    let sum1: i32 = map.map.iter().map(|row| {
        row.iter().filter(|&&ch| ch == 'X').count() as i32
    }).sum();

    // println!("map: {:?}", map);
    println!("sum1: {}", sum1);
    println!("sum2: {}", sum2);
}
