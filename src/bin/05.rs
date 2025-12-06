use itertools::{Itertools};

use rand::seq::SliceRandom;
use rand::rng;

#[aoc::main(05)]
fn main(input: &str) -> (usize, usize) {
    let (raw_ranges, raw_ingredients) = input.split_once("\n\n").unwrap();

    let ranges = raw_ranges.split('\n')
        .map(|range| range.split_once('-').unwrap())
        .map(|(first, second)| (first.parse::<u64>().unwrap(), second.parse::<u64>().unwrap()))
        .collect_vec();

    let ingredients = raw_ingredients.split('\n').map(|ingredient| ingredient.parse::<u64>().unwrap()).collect_vec();

    let mut count_fresh = 0;

    for ingredient in ingredients {
        for (start, end) in &ranges {
            if ingredient >= *start && ingredient <= *end {
                count_fresh += 1;
                break;
            }
        }
    }

    let mut updated_ranges = ranges.clone();
    // random shuffle to make sure no off by one errors
    updated_ranges.shuffle(&mut rng());

    'main: loop {
        'range_loop: for index in 0..updated_ranges.len() {
            let range = updated_ranges[index];
            let overlapping_ranges = get_overlapping_ranges(&updated_ranges, &range, index);

            if overlapping_ranges.len() == 0 {
                continue
            }

            let mut highest_relevant_value = range.1;
            let mut lowest_relevant_value = range.0;

            for (start, end) in overlapping_ranges {
                if range.0 >= start && range.1 <= end {
                    // we're entirely covered inside another range, ignore us!
                    println!("deleting {range:?} {start}, {end}");
                    updated_ranges.remove(index);
                    continue 'main
                }

                if start >= range.0 && end <= range.1 {
                    // it's entirely inside of us now, ignore this one
                    continue 'range_loop
                }

                if range.0 > start && range.1 > end {
                    lowest_relevant_value = end + 1;
                    break;
                }

                if range.1 < end && range.0 < start {
                    highest_relevant_value = start - 1;
                    break;
                }
            }

            println!("downsizing from {range:?} to ({lowest_relevant_value}, {highest_relevant_value})");
            updated_ranges[index] = (lowest_relevant_value, highest_relevant_value);
            continue 'main
        }
        break
    }

    let mut count_ingredients = 0;

    println!("{updated_ranges:?}");
    for (start, end) in updated_ranges {
        count_ingredients += (end - start) + 1;
    }

    (count_fresh, count_ingredients as usize)
}

fn get_overlapping_ranges(ranges: &Vec<(u64, u64)>, range: &(u64, u64), index: usize) -> Vec<(u64, u64)> {
    let mut overlapping_ranges = Vec::new();

    for other_index in 0..ranges.len() {
        if other_index == index {
            continue
        }

        let (start, end) = ranges[other_index];

        if (range.0 >= start && range.0 <= end) || (range.1 >= start && range.1 <= end) {
            overlapping_ranges.push((start, end));
        }
    }

    overlapping_ranges
}
