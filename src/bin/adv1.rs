use std::fs;

fn main() {
    let content = fs::read_to_string("./input1.txt").expect("cannot read file");

    let depths = content
        .lines()
        .map(|x| x.parse().expect("expected a number"))
        .collect::<Vec<i32>>();


    let mut count = 0;

    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            count = count + 1;
        }
    }

    println!("increased {} times", count);

    let mut count = 0;
    let mut prev_window = depths[0] + depths[1] + depths[2];

    for i in 3..depths.len() {
        let window = prev_window - depths[i - 3] + depths[i];
        if window > prev_window {
            count = count + 1
        }
        prev_window = window
    }

    println!("sliding window increased {} times", count);
}
