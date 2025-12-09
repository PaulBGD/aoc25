use std::cmp::{max, min};
use std::ops::Deref;
use hashbrown::HashSet;
use itertools::Itertools;

#[aoc::main(09)]
fn main(input: &str) -> (usize, usize) {
    let xs = input.split('\n').map(|l| {
        let (one, two) = l.split_once(',').unwrap();
        return (one.parse::<u64>().unwrap(), two.parse::<u64>().unwrap())
    }).collect::<Vec<_>>();

    let largest = xs.iter().combinations(2).map(|vec| {
        let (x1, y1) = *vec[0];
        let (x2, y2) = *vec[1];

        let size = ((max(x1, x2) - min(x1, x2)) + 1) * ((max(y2, y1) - min(y1, y2)) + 1);

        return size
    }).sorted().rev().nth(0).unwrap();

    // reduce the size for our flood fill
    // let min_width = xs.iter().sorted_by(|(x1, _), (x2, _)| x1.cmp(x2)).nth(0).unwrap().0;
    // let min_height = xs.iter().sorted_by(|(_, y1), (_, y2)| y1.cmp(y2)).nth(0).unwrap().1;
    // println!("reducing height/width by {min_width} {min_height}");
    //
    // let resized = xs.iter().map(|(x, y)| (x - min_width, y - min_height)).collect_vec();
    // println!("{resized:?}");

    let width = xs.iter().sorted_by(|(x1, _), (x2, _)| x2.cmp(x1)).nth(0).unwrap().0;
    let height = xs.iter().sorted_by(|(_, y1), (_, y2)| y2.cmp(y1)).nth(0).unwrap().1;
    println!("width: {width}, height: {height}");

    let mut all_valid = HashSet::new();
    xs.iter().for_each(|val| {all_valid.insert(*val);});

    // draw lines
    for index in 0..xs.len() {
        let curr = xs[index];
        let next = if index == xs.len() - 1 { xs[0] } else { xs[index + 1] };

        if curr.0 != next.0 {
            // move on x-axis
            let smaller = min(curr.0, next.0);
            let largest = max(curr.0, next.0);
            for x in smaller..largest + 1 {
                all_valid.insert((x, curr.1));
            }
        } else {
            // move on y-axis
            let smaller = min(curr.1, next.1);
            let largest = max(curr.1, next.1);
            for y in smaller..largest + 1 {
                all_valid.insert((curr.0, y));
            }
        }
    }

    // i have never had to do a problem like this so here's the game plan
    // we need to find a spot either on the inside or outside.
    // i think it's easy to find a spot on the inside by taking a corner, and moving +1 in each direction of its lines
    //  and checking that it doesn't overlap any existing points
    // then basic flood fill, pray we don't run out of memory, and brute force
    let to_start = find_starting_fill_spot(&xs, &all_valid, width, height);
    println!("probably valid {to_start:?}");

    let mut to_check = Vec::new();
    to_check.push(to_start);

    while to_check.len() > 0 {
        let node = to_check.pop().unwrap();
        if !all_valid.insert(node) {
            continue;
        }

        to_check.push((node.0 - 1, node.1));
        to_check.push((node.0 + 1, node.1));
        to_check.push((node.0, node.1 - 1));
        to_check.push((node.0, node.1 + 1));
        let len = to_check.len();

        if len % 1000000 == 0 {
            println!("{len}");
        }
    }


    let len = all_valid.len();
    println!("now have {len}");

    (largest as usize, 0)
}

fn find_starting_fill_spot(points: &Vec<(u64, u64)>, valid: &HashSet<(u64, u64)>, width: u64, height: u64) -> (u64, u64) {
    for index in 1..points.len() {
        let previous = points[index - 1];
        let curr = points[index];
        let next = points[index + 1];

        // find which direction goes which way
        let x_diff = if previous.0 == curr.0 { next } else { previous};
        let y_diff = if previous.1 == curr.1 { next } else { previous};

        let to_check = (
            if x_diff.0 < curr.0 { curr.0 - 1 } else {curr.0 + 1},
            if y_diff.1 < curr.1 { curr.1 - 1 } else {curr.1 + 1}
        );

        if !valid.contains(&to_check) {
            println!("found form {previous:?} {curr:?} {next:?}");
            return to_check
        }
    }

    unreachable!("no good?");
}
