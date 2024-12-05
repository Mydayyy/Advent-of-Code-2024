use aoc24::{get_input, parse_input_matrix};
use std::collections::HashMap;

fn main() {
    let input = get_input(4, false);

    let puzzle = parse_input_matrix(input);

    let mut sum1 = 0;
    let mut sum2 = 0;

    let dirs = [
        (-1, -1i32),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let needle = "XMAS";
    let needle_first = needle.chars().next().unwrap();

    for y in 0..puzzle.len() {
        for x in 0..puzzle[y].len() {
            if puzzle[y][x] != needle_first {
                continue;
            }
            for (dx, dy) in dirs {
                let mut found = true;
                for i in 1..needle.len() {
                    let nx = x as i32 + dx * i as i32;
                    let ny = y as i32 + dy * i as i32;
                    if nx < 0 || nx >= puzzle[y].len() as i32 || ny < 0 || ny >= puzzle.len() as i32
                    {
                        found = false;
                        break;
                    }
                    if puzzle[ny as usize][nx as usize] != needle.chars().nth(i).unwrap() {
                        found = false;
                        break;
                    }
                }
                if found {
                    sum1 += 1;
                }
            }
        }
    }
    println!("sum1: {}", sum1);

    let valid_hashmap = HashMap::from([('S', 'M'), ('M', 'S')]);

    for y in 0..puzzle.len() {
        for x in 0..puzzle[y].len() {
            if puzzle[y][x] != 'A' {
                continue;
            }
            let inner_board = y > 0 && y < puzzle.len() - 1 && x > 0 && x < puzzle[y].len() - 1;
            if !inner_board {
                continue;
            }
            let corners = [
                (puzzle[y - 1][x - 1], puzzle[y + 1][x + 1]),
                (puzzle[y - 1][x + 1], puzzle[y + 1][x - 1]),
            ];

            let found = corners
                .iter()
                .all(|(c1, c2)| valid_hashmap.get(c1) == Some(c2));

            if found {
                sum2 += 1;
            }
        }
    }
    println!("sum2: {}", sum2);
}
