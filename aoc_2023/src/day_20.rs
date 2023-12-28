use common::{ Answer, Solution };

use std::collections::{ HashMap, VecDeque };

pub struct Day20;

impl Solution for Day20 {
    fn name(&self) -> &'static str {
        "Pulse Propagation"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve2(input).into();
    }
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    kind: String,
    state: bool,  // Flip flop only
    inputs: HashMap<String, bool>,  // Conjunction only
    outputs: VecDeque<String>
}

fn parse(input: &str) -> HashMap<String, Module> {
    let mut modules = HashMap::<String, Module>::new();

    for line in input.lines() {
        let (left, right) = line.split_once(" -> ").unwrap();

        let outputs = right.split(", ").map(|s| s.to_string()).collect::<VecDeque<String>>();

        let m: Module;
        if left == "broadcaster" {
            m = Module {
                name: "Broadcaster".to_string(),
                kind: "".to_string(),
                state: false,
                inputs: HashMap::<String, bool>::new(),
                outputs
            };
        } else {
            let left = left.to_string();
            let kind = &left[0..1];
            let name = &left[1..];

            m = Module {
                name: name.to_string(),
                kind: kind.to_string(),
                state: false,
                inputs: HashMap::<String, bool>::new(),
                outputs
            };
        }
        modules.insert(m.name.to_string(), m);
    }

    // Get inputs for Conjunction memory
    let others = modules.clone();
    for (_, module) in modules.iter_mut() {
        if module.kind == "&" {
            for (_, other) in others.iter() {
                if other.outputs.contains(&module.name) {
                    module.inputs.insert(other.name.clone(), false);
                }
            }
        }
    }

    return modules;
}


fn solve(input: &str) -> u64 {
    let mut modules = parse(input);

    let mut lo = 0;
    let mut hi = 0;

    for _ in 0..1000 {
        // origin,target, pulse
        let mut q = VecDeque::<(String, String, bool)>::new();

        for (name, m) in modules.iter() {
            if name == "Broadcaster" {
                for other_name in m.outputs.iter() {
                    q.push_back((name.to_string(), other_name.to_string(), false))
                }
            }
        }

        lo += 1;

        while !q.is_empty() {
            let (origin, target, pulse) = q.pop_front().unwrap();
            if pulse { hi += 1 } else { lo += 1 }

            if ! modules.contains_key(&target) { continue }

            if modules.get(&target).unwrap().kind == "%" {
                if !pulse {
                    modules.get_mut(&target).unwrap().state = if ! modules.get(&target).unwrap().state { true } else { false };

                    let outgoing = modules.get(&target).unwrap().state;
                    for output in modules.get(&target).unwrap().outputs.iter() {
                        q.push_back((modules.get(&target).unwrap().name.to_string(), output.to_string(), outgoing))
                    }
                }
            }
            if modules.get(&target).unwrap().kind == "&" {
                modules.get_mut(&target).unwrap().inputs.insert(origin, pulse);

                let outgoing = !modules.get(&target).unwrap().inputs.iter().all(|(_name, s)| *s);
                for output in modules.get(&target).unwrap().outputs.iter() {
                    q.push_back((modules.get(&target).unwrap().name.to_string(), output.to_string(), outgoing))
                }
            }
        }

    }

    return lo * hi;
}


fn solve2(input: &str) -> u64 {
    let mut modules = parse(input);

    // Assuming, only one module feeds to 'rx'
    let mut feed = String::from("");
    for (name, module) in modules.iter() {
        if module.outputs.contains(&"rx".to_string()) {
            feed = name.to_string();
            break
        }
    }

    let mut seen = HashMap::<String, u64>::new();
    for (name, module) in modules.iter() {
        if module.outputs.contains(&feed) {
            seen.insert(name.to_string(), 0);
        }
    }

    for presses in 1.. {
        // origin, target, pulse
        let mut q = VecDeque::<(String, String, bool)>::new();

        for (name, m) in modules.iter() {
            if name == "Broadcaster" {
                for other_name in m.outputs.iter() {
                    q.push_back((name.to_string(), other_name.to_string(), false))
                }
            }
        }

        while !q.is_empty() {
            let (origin, target, pulse) = q.pop_front().unwrap();

            if ! modules.contains_key(&target) { continue }

            // Find first instance when high was sent to conjunction that sends to `rx`
            // - is also an assumption, puzzle just works this way -- must not be the case!!!
            if &modules.get(&target).unwrap().name == &feed && pulse {
                if seen.get(&origin).unwrap() == &0 {
                    *seen.get_mut(&origin).unwrap() = presses;
                };
            }

            if seen.values().all(|&x| x > 0) {
                // find lcm for all cycles when high was sent to conjunction that can trigger rx
                return lcm(&Vec::from_iter(seen.values().map(|x| *x as usize))) as u64;
            }

            // Play game
            if modules.get(&target).unwrap().kind == "%" {
                if !pulse {
                    modules.get_mut(&target).unwrap().state = if ! modules.get(&target).unwrap().state { true } else { false };

                    let outgoing = modules.get(&target).unwrap().state;
                    for output in modules.get(&target).unwrap().outputs.iter() {
                        q.push_back((modules.get(&target).unwrap().name.to_string(), output.to_string(), outgoing))
                    }
                }
            }
            if modules.get(&target).unwrap().kind == "&" {
                modules.get_mut(&target).unwrap().inputs.insert(origin, pulse);

                let outgoing = !modules.get(&target).unwrap().inputs.iter().all(|(_name, s)| *s);
                for output in modules.get(&target).unwrap().outputs.iter() {
                    q.push_back((modules.get(&target).unwrap().name.to_string(), output.to_string(), outgoing))
                }
            }
        }

    }
    return 0;
}

// SRC: https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE), 11687500);
    }
}