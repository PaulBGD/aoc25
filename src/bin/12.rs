use itertools::Itertools;

#[aoc::main(12)]
fn main(input: &str) -> (usize, usize) {
    let sections = input.split("\n\n").collect_vec();
    let areas = sections[sections.len() - 1];

    let part_1 = areas.split('\n').filter(|area| {
        let (lhs, rhs) = area.split_once(": ").unwrap();
        let (width_str, height_str) = lhs.split_once('x').unwrap();
        let width = width_str.parse::<usize>().unwrap();
        let height = height_str.parse::<usize>().unwrap();

        let count_requirements: usize = rhs.split(' ')
            .map(|num| num.parse::<usize>().unwrap()).sum();

        println!("{width} {height} {count_requirements}");
        return count_requirements * 9 <= (width * height) || (width * height) >= count_requirements * 7
    }).count();


    (part_1, 0)
}
