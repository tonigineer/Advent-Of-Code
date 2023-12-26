use common::{ Answer, Solution };
use itertools::Itertools;

use std::collections::{ HashMap, HashSet };

pub struct Day19;

impl Solution for Day19 {
    fn name(&self) -> &'static str {
        "Aplenty"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve2(input).into();
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64
}

impl Part {
    fn new(x: u64, m: u64, a: u64, s: u64) -> Part {
        return Part { x, m, a, s }
    }
}

#[derive(Debug)]
struct Workflow {
    _name: String,
    rules: Vec<(String, String, u64, String)>,
    default: String
}

fn parse(input: &str) -> (HashSet<Part>, HashMap<String, Workflow>) {
    let mut parts = HashSet::<Part>::new();
    let mut workflows = HashMap::<String, Workflow>::new();

    let (first, second) = input.split_once("\n\n").unwrap();

    for line in second.lines() {
        let ratings = line.trim_end_matches('}').trim_start_matches('{').split(',')
            .map(|s| s[2..].parse::<u64>().unwrap()).collect::<Vec<u64>>();
        parts.insert(Part::new(ratings[0], ratings[1], ratings[2], ratings[3]));
    }

    for line in first.lines() {
        let (name, steps) = line.trim_end_matches('}').split_once('{').unwrap();

        let name = name.to_string();
        let mut default = String::new();
        let mut rules = Vec::<(String, String, u64, String)>::new();

        for r in steps.split(',') {
            // println!("{}", r);
            if r.contains(':') {
                let (front, back) = r.split_once(':').unwrap();
                rules.push((
                        front.chars().nth(0).unwrap().to_string(),
                        front.chars().nth(1).unwrap().to_string(),
                        front.chars().skip(2).join("").parse::<u64>().unwrap(),
                        back.to_string()
                ))
            } else {
                default = r.to_string();
            }
        }

        let wf = Workflow{ _name: name.clone(), rules, default };
        workflows.insert(name, wf);
    }


    return (parts, workflows);
}

fn solve(input: &str) -> u64 {
    let (parts, workflows) = parse(input);
    let mut ans = 0;

    for p in parts {
        let mut next_wf = "in".to_string();

        loop {
            let wf = workflows.get(&next_wf).unwrap();

            let mut use_default = true;
            for rule in wf.rules.iter() {
                let rating = match rule.0.as_str() {
                    "x" => p.x,
                    "m" => p.m,
                    "a" => p.a,
                    "s" => p.s,
                    _ => panic!("Not supposed to happen, ffs.")
                };

                let result = match rule.1.as_str() {
                    "<" => rating < rule.2,
                    ">" => rating > rule.2,
                    _ => panic!("Not supposed to happen, ffs.")
                };

                if result {
                    next_wf = rule.3.clone();
                    use_default = false;
                    break;
                }
            }

            if use_default { next_wf = wf.default.clone() }
            if next_wf == "A" || next_wf == "R" { break }
        }
        if next_wf == "A" {
            ans += p.x + p.m + p.a + p.s;
        }
    }
    return ans;
}

fn count(workflows: &HashMap<String, Workflow>, mut ranges: HashMap<String, (u64, u64)>, wf_name: String) -> u64 {
    if wf_name == "R" { return 0 };

    if wf_name == "A" { 
        let mut product = 1;
        for (_, (lo, hi)) in ranges.iter() {
            product *= hi - lo + 1;
        }
        return product;
    }

    let wf = workflows.get(&wf_name).unwrap();

    let mut use_default = true;
    let mut ans: u64 = 0;

    for rule in wf.rules.iter() {
        let (lo, hi) = ranges.get(&rule.0).unwrap();

        let (true_range, false_range) = match rule.1.as_str() {
            "<" => ((*lo, rule.2-1), (rule.2, *hi)),
            ">" => ((rule.2+1, *hi), (*lo, rule.2)),
            _ => panic!("Not supposed to happen, ffs.")
        };

        if true_range.0 <= true_range.1 {
            let mut new_range = ranges.clone();
            new_range.insert(rule.0.clone(), true_range);
            ans += count(&workflows, new_range, rule.3.clone());
        }
        if false_range.0 <= false_range.1 {
            ranges.insert(rule.0.clone(), false_range);
        } else {
            use_default = false;
            break
        }
    }

    if use_default {
        ans += count(&workflows, ranges, wf.default.clone());
    }

    return ans;
}

fn solve2(input: &str) -> u64 {
    let (_, workflows) = parse(input);

    let mut ranges = HashMap::<String, (u64, u64)>::new();
    ranges.insert(String::from("x"), (1, 4000));
    ranges.insert(String::from("m"), (1, 4000));
    ranges.insert(String::from("a"), (1, 4000));
    ranges.insert(String::from("s"), (1, 4000));

    return count(&workflows, ranges, String::from("in"));
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::{solve, solve2};

    const SAMPLE: &str = indoc! {"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE), 19_114);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve2(SAMPLE), 167_409_079_868_000);
    }
}