use std::collections::HashMap;

#[aoc::main(07)]
fn main(input: &str) -> (usize, usize) {
    let xs = input.split('\n').collect::<Vec<_>>();

    let s_index = xs[0].find('S').unwrap();
    let mut collisions = HashMap::new();
    let unique_paths = iterate_beam(&xs, 0, s_index, &mut collisions);

    (collisions.len(), unique_paths)
}

fn iterate_beam(rows: &Vec<&str>, row: usize, column: usize, collisions: &mut HashMap<(usize, usize), usize>) -> usize  {
    if row + 1 >= rows.len() {
        return 1
    }

    let next_row = rows[row + 1];

    let char = next_row.chars().nth(column).unwrap();

    if char == '.' {
        return iterate_beam(rows, row + 1, column, collisions);
    }

    if char == '^' {
        let tuple = (row + 1, column);
        if collisions.contains_key(&tuple) {
            return *collisions.get(&tuple).unwrap();
        }

        let trees = iterate_beam(rows, row, column - 1, collisions) + iterate_beam(rows, row, column + 1, collisions);
        collisions.insert(tuple, trees);
        return trees
    }

    panic!("no good?");
}
