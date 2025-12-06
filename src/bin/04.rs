#[aoc::main(04)]
fn main(input: &str) -> (usize, usize) {
    let xs = input.split('\n').map(|str| str.to_owned()).collect::<Vec<_>>();

    let mut part_1 = 0;

    for y in 0..xs.len() {
        let row = xs[y].clone();
        for x in 0..row.len() {
            let char = get_from_position(&xs, x as i32, y as i32);


            if char == '@' && check_valid(&xs, x as u32, y as u32) {
                part_1 += 1;
                print!("x");
            } else {
                print!("{}", char);
            }
        }
        println!();
    }

    let mut part_2 = 0;
    let mut last_part_2_value = 0;
    let mut vec_copy = xs.clone();

    loop {
        for y in 0..vec_copy.len() {
            let row = &vec_copy[y];
            for x in 0..row.len() {
                let char = get_from_position(&vec_copy, x as i32, y as i32);

                if char == '@' && check_valid(&vec_copy, x as u32, y as u32) {
                    part_2 += 1;
                    print!("x");

                    let row = &mut vec_copy[y];
                    row.replace_range(x..x + 1, "x");
                } else {
                    print!("{}", char);
                }
            }
            println!();
        }

        if part_2 == last_part_2_value {
            break
        }
        last_part_2_value = part_2;
    }

    (part_1, part_2)
}

fn check_valid(vec: &Vec<String>, x: u32, y: u32) -> bool {
    let mut other_rolls = 0;

    for x_to_check in (x as i32 - 1)..(x as i32 + 2) {
        for y_to_check in (y as i32 - 1)..(y as i32 + 2) {
            if x_to_check == x as i32 && y_to_check == y as i32 {
                continue
            }

            if get_from_position(&vec, x_to_check, y_to_check) == '@' {
                // println!("found char at {x_to_check} {y_to_check}");
                other_rolls += 1;

                if other_rolls >= 4 {
                    // print!("{other_rolls}");
                    return false
                }
            }
        }
    }

    true
}

fn get_from_position(vec: &Vec<String>, x: i32, y: i32) -> char {
    if y < 0 || y >= vec.len() as i32 {
        return '.'
    }

    let str = vec[y as usize].clone();

    if x < 0 || x >= str.len() as i32 {
        return '.'
    }

    str.chars().nth(x as usize).unwrap()
}