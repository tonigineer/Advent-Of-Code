use common::{Answer, Solution};
use itertools::Itertools;
use std::collections::HashMap;

pub struct Day24;

impl Solution for Day24 {
    fn name(&self) -> &'static str {
        "Crossed Wires"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input).into()
    }

    fn part2(&self, _input: &str) -> Answer {
        // Solved by hand
        "gjc,gvm,qjj,qsb,wmp,z17,z26,z39".into()
    }
}

fn operate<'a>(
    wires: &mut HashMap<&'a str, bool>,
    formulas: &HashMap<&str, (&'a str, &str, &'a str)>,
    wire: &'a str,
) -> bool {
    if let Some(&value) = wires.get(wire) {
        return value;
    }

    let (a, op, b) = formulas[wire];
    let v1 = operate(wires, formulas, a);
    let v2 = operate(wires, formulas, b);

    let value = match op {
        "AND" => v1 && v2,
        "XOR" => v1 != v2,
        "OR" => v1 || v2,
        _ => unreachable!(),
    };

    wires.insert(wire, value);
    value
}

fn combine(wires: &HashMap<&str, bool>, prefix: &str) -> usize {
    let mut result = 0;
    for wire in wires.keys() {
        if !wire.starts_with(prefix) {
            continue;
        }
        let num = wire[1..].parse::<usize>().unwrap();
        if wires[wire] {
            println!("{:?}", wire);
            result |= 1 << num;
        }
    }

    result
}

fn solve(input: &str) -> usize {
    let (str_wires, str_formulas) = input.split_once("\n\n").unwrap();
    let mut wires = HashMap::new();
    let mut formulas = HashMap::new();

    for line in str_wires.lines() {
        let (name, value) = line.split_once(": ").unwrap();
        wires.insert(name, value == "1");
    }

    for line in str_formulas.lines() {
        let (a, op, b, _, c) = line.split_whitespace().collect_tuple().unwrap();
        formulas.insert(c, (a, op, b));
    }

    for wire in formulas.keys() {
        operate(&mut wires, &formulas, wire);
    }

    combine(&wires, "z")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
        x00: 1
        x01: 1
        x02: 1
        y00: 0
        y01: 1
        y02: 0

        x00 AND y00 -> z00
        x01 XOR y01 -> z01
        x02 OR y02 -> z02
    "};

    const SAMPLE2: &str = indoc::indoc! {"
        x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj
    "};

    #[test]
    fn example_part1a() {
        assert_eq!(solve(SAMPLE), 4);
    }

    #[test]
    fn example_part1b() {
        assert_eq!(solve(SAMPLE2), 2024);
    }
}
