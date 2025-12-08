use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

#[aoc::main(08)]
fn main(input: &str) -> (usize, usize) {
    let xs = input
        .split('\n')
        .map(|l| {
            let split = l
                .split(',')
                .map(|val| val.parse::<usize>().unwrap())
                .collect_vec();

            return (split[0], split[1], split[2]);
        })
        .collect::<Vec<_>>();

    let grids = &mut xs
        .iter()
        .map(|item| {
            let mut vec = Vec::new();
            vec.push(item.clone());
            vec
        })
        .collect_vec();

    let distances = xs.iter()
        .map(|junction| {
        return xs.iter()
            .filter(|other_junc| **other_junc != *junction)
            .map(|other_junc| (*junction, *other_junc))
            .collect_vec();
    })
        .flatten()
        .map(get_correct_order_junctions)
        .unique()
        .sorted_by(|pair_a, pair_b| {
            let distance_a = distance(pair_a.0, pair_a.1);
            let distance_b = distance(pair_b.0, pair_b.1);

            return distance_a.cmp(&distance_b);
        })
        .collect_vec();

    let mut direct_connections = HashMap::new();
    // preload
    for junction in &xs {
        direct_connections.insert(junction, RefCell::new(HashSet::new()));
    }

    // keep finding the grids with the closest match
    for pair in distances {
        let closest_a = pair.0;
        let closest_b = pair.1;
        let grid_index_a = find_in_existing_grid(&grids, closest_a);
        let grid_index_b = find_in_existing_grid(&grids, closest_b);

        let mut conns_a = direct_connections.get(&closest_a).unwrap().borrow_mut();
        conns_a.insert(closest_b);
        let mut conns_b = direct_connections.get(&closest_b).unwrap().borrow_mut();
        conns_b.insert(closest_a);

        if grid_index_a == grid_index_b {
            continue;
        }

        let mut vec_a = grids[grid_index_a].clone();
        let mut vec_b = grids[grid_index_b].clone();
        // println!("added {vec_a:?} to {vec_b:?}");

        vec_a.append(&mut vec_b);
        grids[grid_index_a] = vec_a;
        grids.remove(grid_index_b);

        if grids.len() == 1 {
            return (0, closest_a.0 * closest_b.0)
        }
    }
    panic!()

    // part 1 stuff.. to clean up if i feel like it
    // let sorted_grids = grids
    //     .iter()
    //     .sorted_by(|a, b| b.len().cmp(&a.len()))
    //     .collect_vec();
    // let top_grids = &sorted_grids[0..3];
    //
    // let grid_size = top_grids
    //     .iter()
    //     .map(|vec| vec.len())
    //     .reduce(|acc, curr| acc * curr)
    //     .unwrap();
    //
    // (grid_size, 0)
}

fn get_correct_order_junctions((j1, j2): ((usize, usize, usize), (usize, usize, usize))) -> ((usize, usize, usize), (usize, usize, usize)) {
    if j1.0 < j2.0 {
        return (j1, j2);
    }
    if j2.0 < j1.0 {
        return (j2, j1)
    }

    if j1.1 < j2.1 {
        return (j1, j2)
    }
    if j2.1 < j1.1 {
        return (j2, j1)
    }

    if j1.2 < j2.2 {
        return (j1, j2)
    }
    if j2.2 < j1.2 {
        return (j2, j1)
    }

    (j2, j1)
}

fn find_in_existing_grid(grids: &Vec<Vec<(usize, usize, usize)>>, junction: (usize, usize, usize)) -> usize {
    for (grid_index,grid) in grids.iter().enumerate() {
        for other_junc in grid {
            if *other_junc == junction {
                return grid_index
            }
        }
    }

    println!("failed to find {junction:?} in {grids:?}");
    unreachable!("no good?");
}

fn distance((ax, ay, az): (usize, usize, usize), (bx, by, bz): (usize, usize, usize)) -> usize {
    ((bx as i128 - ax as i128).pow(2)
        + (by as i128 - ay as i128).pow(2)
        + (bz as i128 - az as i128).pow(2)) as usize
}
