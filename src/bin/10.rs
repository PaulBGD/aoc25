use itertools::Itertools;
use microlp::{ComparisonOp, OptimizationDirection, Problem};
use regex::Regex;
use std::collections::HashSet;

#[aoc::main(10)]
fn main(input: &str) -> (usize, usize) {
    let re = Regex::new(r"\[(?<indicators>[.#]+)] (?<switch>(\((\d,?)+\) )+)\{(?<values>(\d+,?)+)}").unwrap();
    let switch_re = Regex::new(r"(?<switch>(\d,?)+)").unwrap();

    let xs = input.split('\n').map(|l| {
        let captures = re.captures(l).unwrap();
        let indicators = captures.name("indicators").unwrap().as_str();
        let switches = captures.name("switch").unwrap().as_str();
        let values = captures.name("values").unwrap().as_str();

        let switches_captures = switch_re.captures_iter(switches).map(|m| {
            m.name("switch").unwrap().as_str().split(',').map(|num| num.parse::<u8>().unwrap()).collect_vec()
        }).collect_vec();

        let values_captures = values.split(',').map(|val| val.parse::<u32>().unwrap()).collect_vec();

        return Machine {
            indicators,
            switches: switches_captures,
            values: values_captures
        }
    }).collect::<Vec<_>>();

    let part_one = xs.iter().map(|machine| solve_machine(machine)).sum();
    let part_two = xs.iter().map(|machine| solve_part_two(machine)).sum();

    (part_one, part_two)
}

fn solve_machine(machine: &Machine) -> usize {
    for max_combinations_to_try in 1..999 {
        let all_combos: HashSet<Vec<&Vec<u8>>> = machine.switches.iter()
            .combinations(max_combinations_to_try)
            .map(|vec| {
                let mut cloned = vec.clone();
                cloned.sort();
                return cloned
            })
            .collect();
        let len = all_combos.len();
        println!("trying combos of {len}");

        for option in all_combos {
            let mut indicator = ".".repeat(machine.indicators.len());

            for operation in option {
                for switch in operation {
                    let u = *switch as usize;
                    let replace_with = if indicator.chars().nth(u).unwrap() == '.' { "#" } else { "." };
                    indicator.replace_range(u..u + 1, replace_with);
                }

                if indicator == machine.indicators {
                    println!("found in {max_combinations_to_try}");
                    return max_combinations_to_try;
                }
            }
        }
    }

    unreachable!("no good?");
}

fn solve_part_two(machine: &Machine) -> usize {
    let mut problem = Problem::new(OptimizationDirection::Minimize);
    let vars = machine.switches.iter().map(|switch| {
        let var = problem.add_integer_var(1.0, (0, i32::MAX));
        return (var, switch);
    }).collect_vec();

    for (index, value) in machine.values.iter().enumerate() {
        // we need a constraint that all those vars add up to this value
        let constraints = vars.iter().filter(|(_, switch)| {
            switch.contains(&(index as u8))
        }).map(|(var, _)| {
            (*var, 1.0)
        }).collect_vec();

        problem.add_constraint(
            &constraints,
            ComparisonOp::Eq, *value as f64
        );
    }

    let solution = problem.solve().unwrap();
    // println!("for {machine:?} got solve {solution:?}");

    solution.objective().round() as usize
}

#[derive(Debug)]
struct Machine<'a> {
    indicators: &'a str,
    switches: Vec<Vec<u8>>,
    values: Vec<u32>
}
