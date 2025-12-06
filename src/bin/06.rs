use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

#[aoc::main(06)]
fn main(input: &str) -> (usize, usize) {
    let re = Regex::new(r"  +").unwrap();

    let destringified = re.replace_all(input, " ");
    let xs = destringified.split('\n').map(|l| {
        l.trim().split(' ').collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    println!("{xs:?}");

    let mut rows_by_index = HashMap::new();

    for row in &xs[0..xs.len() - 1] {
        for index in 0..row.len() {
            if !rows_by_index.contains_key(&index) {
                rows_by_index.insert(index, Vec::new());
            }

            let row_editing = rows_by_index.get(&index).unwrap();

            let val = row[index].parse::<u64>().unwrap();

            rows_by_index.insert(index, row_editing.iter().chain(&[val]).cloned().collect());
        }
    }

    let operators = xs[xs.len() - 1].iter().map(|string| string.chars().nth(0).unwrap()).collect_vec();

    let part_1: u64 = rows_by_index.iter().map(|(index, values)| {
        let operator = operators[*index];

        let first_value = values[0];
        let value = values[1..].iter().fold(first_value, |acc, &e| {
            match operator {
                '+' => acc + e,
                '*' => acc * e,
                _ => panic!("no good?") }
        });
        println!("{value}");

        return value;
    }).sum();

    println!("{rows_by_index:?}");

    // probably a better way to do this, but invert!

    (part_1 as usize, 0)
}

#[derive(Debug)]
struct Row {
    nums: Vec<u32>,
    operation: char
}
