

#[aoc::main(02)]
fn main(input: &str) -> (usize, usize) {
    let xs = input.split(',').map(|l| {
        l.split_once('-').unwrap()
    }).collect::<Vec<_>>();

    let mut total_invalid = 0;
    let mut total_invalid_multi = 0;

    for (first, second) in &xs {
        let first_value = first.parse::<i64>().unwrap();
        let second_value = second.parse::<i64>().unwrap();

        for value in first_value..(second_value + 1) {
            let value_as_string = value.to_string();

            // for part one
            let (left, right) = value_as_string.split_at(value_as_string.len() / 2);

            if left == right {
                total_invalid += value;
            }

            // for part two
            for index in 1..value_as_string.len() {
                let (slice, _) = value_as_string.split_at(index);
                let occurrences = value_as_string.matches(slice).count();
                // println!("checking {slice} against {value_as_string} which has {occurrences} occurrences");

                if occurrences * slice.len() == value_as_string.len() {
                    total_invalid_multi += value;
                    break;
                }
            }
        }
    }

    (total_invalid as usize, total_invalid_multi as usize)
}
