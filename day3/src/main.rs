use std::cmp::max;
use std::collections::BTreeMap;
use std::str::MatchIndices;

fn main() {
    let input = std::fs::read_to_string("day3/input.txt").unwrap();
    let potential_matches = input.match_indices("mul(");
    let chars = input.chars().collect::<Vec<_>>();
    let dos = input.match_indices("do()");
    let donts = input.match_indices("don't()");
    let mut active_map = BTreeMap::new();
    active_map.append(&mut BTreeMap::from_iter(
        dos.map(|(idx, _)| (idx, true))
            .chain(donts.map(|(idx, _)| (idx, false))),
    ));

    // Part 2: Filter our matches based on if there is a do()/don't() instruction before it
    let potential_matches = potential_matches.filter(|(idx, str)| {
        active_map
            .range(..idx)
            .next_back()
            .map(|d| *d.1)
            .unwrap_or(true)
    });

    println!("Found {} matches", potential_matches.clone().count());
    part_1(potential_matches, &chars);
}

fn part_1<'a, I>(potential_matches: I, chars: &[char])
where
    I: Iterator<Item = (usize, &'a str)>,
{
    let mut sum_of_mul = 0;
    for (initial_idx, _) in potential_matches {
        let mut idx = initial_idx + 4;
        let mut first_num_buf = vec![];
        while chars[idx].is_numeric() {
            first_num_buf.push(chars[idx]);
            idx += 1;
        }
        let first_num: usize = first_num_buf
            .into_iter()
            .collect::<String>()
            .parse()
            .unwrap();
        println!("{first_num}");
        if first_num > 999 || chars[idx] != ',' {
            println!(
                "Too long num or wrong next character: {} with num: {first_num}",
                chars[idx],
            );
            continue;
        }
        idx += 1;
        let mut second_num_buf = vec![];
        while chars[idx].is_numeric() {
            second_num_buf.push(chars[idx]);
            idx += 1;
        }
        let second_num: usize = second_num_buf
            .into_iter()
            .collect::<String>()
            .parse()
            .unwrap();
        if chars[idx] != ')' || second_num > 999 {
            continue;
        }
        println!("mul({first_num},{second_num})");
        sum_of_mul += first_num * second_num;
    }
    println!("Sum of multiplications: {}", sum_of_mul);
}
