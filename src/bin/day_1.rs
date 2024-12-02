use aoc24::{parse_input_lines_into_pairs, get_input};

fn main() {
    let input = get_input(1, false);
    let input2 = parse_input_lines_into_pairs(input);

    let (mut lista, mut listb): (Vec<i32>, Vec<i32>) = input2.iter().cloned().unzip();
    lista.sort();
    listb.sort();
    let combined_list: Vec<_> = lista.iter().zip(listb.iter()).collect();
    let list_differences = combined_list.iter().map(|line| (line.0 - line.1).abs()).collect::<Vec<i32>>();
    let sum1 = list_differences.iter().sum::<i32>();


    let mut sum2 = 0;
    for i in 0..lista.len() {
        let mut count = 0;
        for j in 0..listb.len() {
            if lista[i] == listb[j] {
                count += 1;
            }
        }
        sum2 += lista[i] * count;
    }

    println!("{}", sum1);
    println!("{}", sum2);
}