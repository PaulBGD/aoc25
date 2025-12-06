use std::cmp::min;
use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

#[aoc::main(06)]
fn main(input: &str) -> (usize, usize) {
    let part_one_re = Regex::new(r"  +").unwrap();

    let destringified = part_one_re.replace_all(input, " ");
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


    let part_two_re = Regex::new(r"[+*] +").unwrap();

    let lines = input.split("\n").collect_vec();

    let character_line = lines[lines.len() - 1];
    let part_2: u64 = part_two_re.find_iter(character_line).map(|m| {
        let operator = m.as_str().chars().nth(0).unwrap();
        let nums = lines[..lines.len() - 1].iter().map(|&line| {
            line[m.start()..min(m.end() - 1, line.len())].to_string()
        }).collect_vec();

        let mut transposed = Vec::new();

        for _ in 0..nums[0].len() {
            transposed.push(String::new());
        }

        for num in nums {
            for (index, char) in num.chars().enumerate() {
                transposed[index].push(char);
            }
        }

        println!("{transposed:?}");

        return transposed.iter().map(|val| val.trim().parse::<u64>().unwrap()).reduce(|acc, curr| {
            match operator {
                '+' => acc + curr,
                '*' => acc * curr,
                _ => panic!("no good?")
            }
        }).unwrap();
    }).sum();

    (part_1 as usize, part_2 as usize)
}
