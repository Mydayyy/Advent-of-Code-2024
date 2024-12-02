use aoc24::{get_input, parse_input_matrix};

fn have_same_sign(a: i32, b: i32) -> bool {
    (a >= 0 && b >= 0) || (a < 0 && b < 0)
}

fn remove_element_at_index(vec: &Vec<i32>, index: usize) -> Vec<i32> {
    let mut new_vec = vec.clone();
    new_vec.remove(index);
    new_vec
}

fn is_report_safe(report: &Vec<i32>, with_deletion: bool) -> bool {
    if with_deletion {
        for i in 0..report.len() {
            let new_report = remove_element_at_index(report, i);
            if is_report_safe(&new_report, false) {
                return true;
            }
        }
        return false;
    }

    if report[0] == report[1] {
        return false;
    }

    let mut initial_diff = report[0] - report[1];

    for i in 0..report.len() - 1 {
        let diff = report[i] - report[i + 1];

        if diff.abs() > 3 || diff == 0 {
            return false;
        }
        if !have_same_sign(initial_diff, diff) {
            return false;
        }
    }

    true
}
fn main() {
    let input = get_input(2, false);
    println!("{:?}", input);

    let reports = parse_input_matrix::<i32>(input);
    println!("{:?}", reports);

    let mut sum1 = 0;
    let mut sum2 = 0;

    sum1 = reports
        .iter()
        .filter(|report| is_report_safe(report, false))
        .count() as i32;
    sum2 = reports
        .iter()
        .filter(|report| is_report_safe(report, true))
        .count() as i32;

    println!("{}", sum1);
    println!("{}", sum2);
}
