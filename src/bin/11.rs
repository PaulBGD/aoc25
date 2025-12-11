use itertools::Itertools;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

#[aoc::main(11)]
fn main(input: &str) -> (usize, usize) {
    let inputs_to_outputs = input
        .split('\n')
        .map(|l| {
            let (input, outputs) = l.split_once(": ").unwrap();
            let outputs = outputs.split(' ').collect_vec();

            return (input, outputs);
        })
        .collect::<HashMap<_, _>>();

    let trees = get_output_tree("you", &inputs_to_outputs);

    let machines = do_thing(&inputs_to_outputs);

    let cache = RefCell::new(HashMap::new());
    let part_2 = get_output_tree_part_2("svr", &inputs_to_outputs, &machines, false, false, &cache);

    (trees.len(), part_2)
}

#[derive(Clone, Debug, Default)]
struct Machine {
    has_fft: bool,
    has_dac: bool,
}

fn do_thing<'a>(inputs_to_outputs: &'a HashMap<&str, Vec<&str>>) -> HashMap<&'a str, Machine> {
    let cache = RefCell::new(HashMap::new());
    do_thing_to_machine("svr", inputs_to_outputs, &cache);

    cache.take()
}

fn get_output_tree<'a>(start: &'a str, map: &'a HashMap<&str, Vec<&str>>) -> HashSet<Vec<&'a str>> {
    let mut set = HashSet::new();

    if start == "out" {
        set.insert(vec![start]);
        return set;
    }

    let outputs = map.get(start).unwrap();

    for output in outputs {
        let mut vec = Vec::new();
        vec.push(start);

        let new_tree = get_output_tree(output, map);
        for new_vec in new_tree {
            let mut cloned = vec.clone();
            cloned.extend_from_slice(&new_vec);
        }
    }

    set
}

// lazy..
fn get_output_tree_part_2<'a>(
    start: &'a str,
    map: &'a HashMap<&str, Vec<&str>>,
    machines: &HashMap<&'a str, Machine>,
    mut has_seen_fft: bool,
    mut has_seen_dac: bool,
    precalculated: &RefCell<HashMap<&'a str, usize>>,
) -> usize {
    if start == "out" {
        return 1;
    }

    if precalculated.borrow().contains_key(start) {
        return *precalculated.borrow().get(start).unwrap();
    }

    has_seen_fft |= start == "fft";
    has_seen_dac |= start == "dac";

    let outputs = map.get(start).unwrap();

    let mut num = 0;
    for output in outputs {
        if (!has_seen_fft || !has_seen_dac) && machines.contains_key(output) {
            let metadata = machines.get(output).unwrap();

            if (!has_seen_fft && !metadata.has_fft) || (!has_seen_dac && !metadata.has_dac) {
                continue; // bad branch
            }
        }

        num += get_output_tree_part_2(
            output,
            map,
            machines,
            has_seen_fft,
            has_seen_dac,
            precalculated,
        );
    }

    precalculated.borrow_mut().insert(start, num);

    num
}

fn do_thing_to_machine<'a>(
    machine: &'a str,
    inputs_to_outputs: &'a HashMap<&str, Vec<&str>>,
    precalculated: &RefCell<HashMap<&'a str, Machine>>,
) -> Machine {
    if !inputs_to_outputs.contains_key(machine) {
        return Machine {
            has_dac: false,
            has_fft: false,
        };
    }

    if precalculated.borrow().contains_key(machine) {
        return precalculated.borrow().get(machine).unwrap().clone();
    }

    let outputs = inputs_to_outputs.get(machine).unwrap();

    let mut has_fft = machine == "fft";
    let mut has_dac = machine == "dac";

    for output in outputs {
        let machine = do_thing_to_machine(output, inputs_to_outputs, &precalculated);
        has_fft |= machine.has_fft;
        has_dac |= machine.has_dac;
    }

    let machine_struct = Machine { has_fft, has_dac };

    precalculated
        .borrow_mut()
        .insert(machine, machine_struct.clone());

    machine_struct
}
