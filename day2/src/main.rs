use std::fs;

fn main() {
    let file = fs::read_to_string("day2/input.txt").unwrap();
    let lines = file.lines();
    // Process to numbers
    let processed_lines = lines
        .map(|s| {
            s.split(" ")
                .map(|num_str| num_str.parse::<u16>().unwrap())
                .collect::<Vec<u16>>()
        })
        .collect::<Vec<_>>();

    let mut counter: usize = 0;
    // This is really, really stupid but it works
    'outer: for nums in processed_lines {
        for idx in 0..nums.len() {
            let mut new_nums = nums.clone();
            new_nums.remove(idx);
            if is_safe(new_nums) {
                counter += 1;
                continue 'outer;
            }
        }
    }
    println!("Counter: {}", counter)
}

fn is_safe(nums: Vec<u16>) -> bool {
    let mut last = nums[0];
    let second_item = nums[1];
    let order = match last.cmp(&second_item) {
        std::cmp::Ordering::Equal => return false,
        o => o,
    };
    let mut iter = nums.iter().skip(1);
    loop {
        if let Some(next) = iter.next() {
            let safe_range: [u16; 3] = [1, 2, 3];
            if last.cmp(next) == order && safe_range.contains(&last.abs_diff(*next)) {
                last = *next;
            } else {
                return false;
            }
        } else {
            return true;
        }
    }
}
