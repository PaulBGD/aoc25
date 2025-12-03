use itertools::concat;

#[aoc::main(03)]
fn main(input: &str) -> (usize, usize) {
    let xs = input.split('\n').collect::<Vec<_>>();

    let mut total_bank = 0;

    for row in &xs {
        let mut highest_number = 0;
        for first_index in 0..(row.len() - 1) {
            for second_index in (first_index + 1)..row.len() {
                let str = format!("{}{}", row.chars().nth(first_index).unwrap(), row.chars().nth(second_index).unwrap());
                let as_num = str.parse::<i32>().unwrap();

                if as_num > highest_number {
                    highest_number = as_num;
                }
            }
        }

        total_bank += highest_number;
    }

    let mut total_bank_p2 = 0;

    for row in &xs {
        // iterate forward until you find the highest number with enough numbers remaining after that you can still form a 12 digit number

        let mut curr_str = String::new();

        let mut highest_index = 0;
        for current_index in 0..row.len() {
            if current_index <= highest_index && highest_index != 0 {
                continue;
            }

            if curr_str.len() == 12 {
                break;
            }

            let remaining_numbers_needed = 12 - curr_str.len();
            let last_index_we_can_use = row.len() - remaining_numbers_needed;

            let current_num = row.chars().nth(current_index).unwrap().to_string().parse::<u8>().unwrap();
            let mut highest_num = current_num;
            highest_index = current_index;

            // look forward
            for next_possibility_index in (current_index + 1)..(last_index_we_can_use + 1) {
                let next_num = row.chars().nth(next_possibility_index).unwrap().to_string().parse::<u8>().unwrap();

                if next_num > highest_num {
                    highest_num = next_num;
                    highest_index = next_possibility_index;
                }
            }

            curr_str = format!("{curr_str}{highest_num}");
        }

        total_bank_p2 += curr_str.parse::<u128>().unwrap();
    }

    (total_bank as usize, total_bank_p2 as usize)
}
