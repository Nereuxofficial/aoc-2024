use std::fs;

fn main() {
    let file = fs::read_to_string("day1/input.txt").unwrap();
    let lines = file.lines();
    let mut left = vec![];
    let mut right = vec![];
    for line in lines {
        if let Some((left_str, right_str)) = line.split_once("   ") {
            let left_num: i32 = left_str.parse().unwrap();
            let right_num: i32 = right_str.parse().unwrap();
            match left.binary_search(&left_num) {
                Ok(pos) => left.insert(pos, left_num),
                Err(pos) => left.insert(pos, left_num),
            }
            match right.binary_search(&right_num) {
                Ok(pos) => right.insert(pos, right_num),
                Err(pos) => right.insert(pos, right_num),
            }
        }
    }
    let total_dif = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (left_num, right_num)| {
            acc + (left_num.abs_diff(*right_num))
        });
    println!("Total diff: {}", total_dif);
    let mut similarity_score = 0;
    for left_num in left {
        for right_num in &right {
            if left_num == *right_num {
                similarity_score += left_num;
            }
        }
    }
    println!("{}", similarity_score)
}
