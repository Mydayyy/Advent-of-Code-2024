use aoc24::get_input;
use regex::Regex;

fn main() {
    let input = get_input(3, false);

    let mut sum1 = 0;
    let mut sum2 = 0;

    {
        let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
        for captures in re.captures_iter(&input) {
            sum1 += captures[1].parse::<i32>().unwrap() * captures[2].parse::<i32>().unwrap();
        }
        println!("{}", sum1);
    }

    {
        let mut enabled = true;
        let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)").unwrap();
        for captures in re.captures_iter(&input) {
            if captures.get(0).unwrap().as_str() == "do()" {
                enabled = true;
            } else if captures.get(0).unwrap().as_str() == "don't()" {
                enabled = false;
            } else if enabled {
                sum2 += captures[1].parse::<i32>().unwrap() * captures[2].parse::<i32>().unwrap();
            }
        }
        println!("{}", sum2);
    }
}
