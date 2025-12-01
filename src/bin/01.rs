#[aoc::main(01)]
fn main(input: &str) -> (usize, usize) {
    let xs = input.split('\n').map(|l| {
        l.split_at(1)
    }).collect::<Vec<_>>();

    let mut current_value = 50;
    let mut times_hit_zero = 0;

    for (direction, value_as_str) in xs.iter() {
        let value = value_as_str.parse::<i32>().unwrap();

        match direction {
            &"L" => current_value = current_value - value,
            &"R" => current_value = current_value + value,
            _ => panic!("no good?")
        }

        while current_value < 0 {
            // println!("less than 0, wrapping {current_value}");
            current_value = 100 + current_value;
            // println!("now have {current_value}");
        }

        while current_value > 99 {
            // println!("more than 99, wrapping {current_value}");
            current_value = current_value - 100;
            // println!("now have {current_value}");
        }

        if current_value == 0 {
            times_hit_zero = times_hit_zero + 1
        }
    }

    current_value = 50; // reset
    let mut times_passed_zero = 0;

    for (direction, value_as_str) in xs.iter() {
        let value = value_as_str.parse::<i64>().unwrap();
        let modifier = if direction == &"L" { -1 } else { 1 };

        // stuck, so let's just try to emulate the exact logic regardless if it's more logic
        // go over each tick
        for _ in 0..value {
            current_value += modifier;

            if current_value % 100 == 0 {
                println!("zero! {current_value}");
                times_passed_zero += 1;
            }
        }
        // this actually ended up being shorter
    }

    (times_hit_zero, times_passed_zero)
}
