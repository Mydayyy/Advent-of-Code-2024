use aoc24::get_input;
use std::collections::HashMap;

fn is_valid_order(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    let res = update.windows(2).all(|w| {
        let a = w[0];
        let b = w[1];
        rules.get(&a).map_or(false, |v| v.contains(&b))
    });
    res
}

fn main() {
    let input = get_input(5, false);

    let sections: Vec<&str> = input.split("\n\n").collect();
    let input_rules = sections[0];
    let input_updates = sections[1];

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    for line in input_rules.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        let key = parts[0].parse::<i32>().unwrap();
        let value = parts[1].parse::<i32>().unwrap();
        let values = rules.entry(key).or_default();
        values.push(value);
    }

    for line in input_updates.lines() {
        let parts: Vec<i32> = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        updates.push(parts);
    }

    let sum1: i32 = updates
        .iter()
        .filter(|update| is_valid_order(update, &rules))
        .map(|update| {
            let middle = update.len() / 2;
            update[middle]
        })
        .sum();

    let sum2: i32 = updates
        .iter_mut()
        .filter(|update| !is_valid_order(update, &rules))
        .map(|update| {
            update.sort_by(|a, b| {
                if !rules.contains_key(a) {
                    std::cmp::Ordering::Greater
                } else if rules.get(a).unwrap().contains(b) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            update
        })
        .map(|update| {
            let middle = update.len() / 2;
            update[middle]
        })
        .sum();

    println!("sum1: {}", sum1);
    println!("sum2: {}", sum2);
}
