use common::{ Answer, Solution };

// NOTE: I have no idea, why z3 does not compile anymore.
//       There i just grabbed the solution from https://github.com/connorslade/advent-of-code/blob/main/aoc_2023/src/day_24.rs
// use z3::ast::{Ast, Int};
// use z3::{Config, Context, Solver};

use std::{fmt::Display, ops::RangeInclusive};
use itertools::Itertools;
use nd_vec::{vector, Vec2, Vec3};
use num_traits::Num;

pub struct Day24;

impl Solution for Day24 {
    fn name(&self) -> &'static str {
        "Never Tell Me The Odds"
    }

    fn part1(&self, input: &str) -> Answer {
        part_a(input)
    }

    fn part2(&self, input: &str) -> Answer {
        part_b(input)
    }
}

fn part_a(input: &str) -> Answer {
    let stones = parse(input);
    solve_a(&stones, 200000000000000.0..=400000000000000.0).into()
}

fn part_b(input: &str) -> Answer {
    let stones = parse(input);

    const BRUTE_RANGE: RangeInclusive<i64> = -1000..=1000;

    let mut possible_x_vel = Vec::new();
    let mut possible_y_vel = Vec::new();
    let mut possible_z_vel = Vec::new();

    let mut iter = stones.iter().tuple_combinations();
    while possible_x_vel.len() != 1 || possible_y_vel.len() != 1 || possible_z_vel.len() != 1 {
        let (a, b) = iter.next().expect("No solution found");
        let process = |possible: &mut Vec<i64>, idx: usize| {
            let pos = (a.pos.as_slice()[idx], b.pos.as_slice()[idx]);
            let vel = (a.vel.as_slice()[idx], b.vel.as_slice()[idx]);

            if vel.0 != vel.1 {
                return;
            }

            let delta = (pos.0 - pos.1).abs();
            let this = BRUTE_RANGE
                .clone()
                .filter(|i| i != &vel.0 && delta % (i - vel.0) == 0)
                .collect_vec();

            possible.retain(|v| this.contains(v));
            if possible.is_empty() {
                possible.extend(this);
            }
        };

        process(&mut possible_x_vel, 0);
        process(&mut possible_y_vel, 1);
        process(&mut possible_z_vel, 2);
    }

    let (a, b) = (stones[0].as_float(), stones[1].as_float());
    let (xv, yv, zv) = (
        possible_x_vel[0] as f64,
        possible_y_vel[0] as f64,
        possible_z_vel[0] as f64,
    );

    let ma = (a.vel.y() - yv) / (a.vel.x() - xv);
    let mb = (b.vel.y() - yv) / (b.vel.x() - xv);

    let ca = a.pos.y() - ma * a.pos.x();
    let cb = b.pos.y() - mb * b.pos.x();

    let x = (cb - ca) / (ma - mb);
    let y = ma * x + ca;
    let t = (x - a.pos.x()) / (a.vel.x() - xv);
    let z = a.pos.z() + (a.vel.z() - zv) * t;

    ((x + y + z) as i64).into()
}

fn parse(input: &str) -> Vec<HailStone<i64>> {
    let mut out = Vec::new();

    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let parse = |i| parts[i as usize].trim_end_matches(',').parse().unwrap();
        out.push(HailStone {
            pos: vector!(parse(0), parse(1), parse(2)),
            vel: vector!(parse(4), parse(5), parse(6)),
        });
    }

    out
}

fn solve_a(stones: &[HailStone<i64>], range: RangeInclusive<f64>) -> usize {
    stones
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| a.collision(b))
        .filter(|&pos| range.contains(&pos.x()) && range.contains(&pos.y()))
        .count()
}

#[derive(Debug, Clone, Copy)]
struct HailStone<T: Num + Copy + Display> {
    pos: Vec3<T>,
    vel: Vec3<T>,
}

impl HailStone<i64> {
    fn as_float(&self) -> HailStone<f64> {
        HailStone {
            pos: self.pos.num_cast().unwrap(),
            vel: self.vel.num_cast().unwrap(),
        }
    }

    fn collision(&self, other: &Self) -> Option<Vec2<f64>> {
        let (a, b) = (self.as_float(), other.as_float());

        let a_slope = a.vel.y() / a.vel.x();
        let a_intercept = a.pos.y() - a_slope * a.pos.x();

        let b_slope = b.vel.y() / b.vel.x();
        let b_intercept = b.pos.y() - b_slope * b.pos.x();

        let x_pos = (b_intercept - a_intercept) / (a_slope - b_slope);
        let y_pos = a_slope * x_pos + a_intercept;

        (x_pos.is_normal()
            && y_pos.is_normal()
            && !(a.vel.x() > 0.0 && x_pos < a.pos.x())
            && !(a.vel.x() < 0.0 && x_pos > a.pos.x())
            && !(b.vel.x() > 0.0 && x_pos < b.pos.x())
            && !(b.vel.x() < 0.0 && x_pos > b.pos.x()))
        .then(|| vector!(x_pos, y_pos))
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::{parse, solve_a};

    const CASE: &str = indoc! {"
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
    "};

    #[test]
    fn part_a() {
        assert_eq!(solve_a(&parse(CASE), 7.0..=37.0), 2);
    }
}

// #[derive(Debug)]
// struct HailStone {
//     x: f64,
//     y: f64,
//     z: f64,
//     vx: f64,
//     vy: f64,
//     vz: f64,
//     a: f64,
//     b: f64,
//     c: f64,
// }

// impl HailStone {
//     fn convert_to_z3<'a>(&self, ctx: &'a Context) -> HailStoneZ3<'a> {
//         HailStoneZ3 {
//             x  : Int::from_i64(ctx, self.x  as i64),
//             y  : Int::from_i64(ctx, self.y  as i64),
//             z  : Int::from_i64(ctx, self.z  as i64),
//             vx : Int::from_i64(ctx, self.vx as i64),
//             vy : Int::from_i64(ctx, self.vy as i64),
//             vz : Int::from_i64(ctx, self.vz as i64),
//         }
//     }
// }

// #[derive(Debug, Clone)]
// struct HailStoneZ3<'a> {
//     x  : Int<'a>,
//     y  : Int<'a>,
//     z  : Int<'a>,
//     vx : Int<'a>,
//     vy : Int<'a>,
//     vz : Int<'a>,
// }

// fn intersection(hs1: &HailStone, hs2: &HailStone, test_limits: bool) -> bool {
//     // 1. convert our line definition to this general form
//     //      ax + by = c
//     //
//     // Px = x + t*vx
//     // Py = y + t*vy
//     //
//     // (Px - x) / vx = t = (Pv - y) / vy
//     //
//     // vy * (Px-x) = vx * (Py-y)
//     //
//     // vy*Px - vx*Py = vy*x - vx*y
//     // >> a = vy, b = -vx, c = vy*x - vx*y
//     //
//     // 2. now find (x, y) to satisfy the system
//     //      a1x + b1y = c1
//     //      a2x + b2y = c2
//     // while using our a, b, c values for the hailstones

//     // Parallel
//     if hs1.a * hs2.b == hs1.b * hs2.a { return false }

//     // Get intersection
//     let x = (hs1.c * hs2.b - hs2.c * hs1.b) /
//         (hs1.a * hs2.b - hs1.b * hs2.a);
//     let y = (hs2.c * hs1.a - hs1.c * hs2.a) /
//         (hs1.a * hs2.b - hs1.b * hs2.a);

//     // Check if within boundaries
//     let x_min: f64 = if test_limits {7.0} else {200000000000000.0};
//     let x_max: f64 = if test_limits {27.0} else {400000000000000.0};
//     let y_min = x_min;
//     let y_max = x_max;

//     if x < x_min || x > x_max || y < y_min || y > y_max { return false }

//     // Check if not in future
//     if hs1.vx * (x - hs1.x) < 0.0 || hs1.vy * (y - hs1.y) < 0.0 { return false }
//     if hs2.vx * (x - hs2.x) < 0.0 || hs2.vy * (y - hs2.y) < 0.0 { return false }

//     return true;
// }

// fn parse(input: &str) -> Vec<HailStone> {
//     let mut hs = Vec::<HailStone>::new();

//     for line in input.lines() {
//         let (pos, vel) = line
//             .split_once(" @ ")
//             .unwrap();
    
//         let p = pos
//             .split(",")
//             .map(|s| s.trim_start_matches(' ').trim_end_matches(' ').parse::<i64>().unwrap())
//             .collect::<Vec<i64>>();

//         let v = vel
//         .split(",")
//         .map(|s| s.trim_start_matches(' ').trim_end_matches(' ').parse::<i64>().unwrap())
//         .collect::<Vec<i64>>();

//         let h = HailStone{
//             x: p[0] as f64,
//             y: p[1] as f64,
//             z: p[2] as f64,
//             vx: v[0] as f64,
//             vy: v[1] as f64,
//             vz: v[2] as f64,
//             a: v[1] as f64,
//             b: -v[0] as f64,
//             c: (v[1] * p[0] - v[0] * p[1]) as f64,
//         };

//         hs.push(h);
//     }

//     return hs;
// }

// fn solve(input: &str, test_limits: bool) -> u64 {
//     let hs = parse(input);

//     let mut ans = 0;
//     for (idx, hs1) in hs.iter().enumerate() {
//         for hs2 in hs[..idx].iter() {
//             if intersection(hs1, hs2, test_limits) { 
//                 ans += 1;
//              }
//         }
//     }

//     return ans;
// }

// fn solve2(input: &str) -> u64 {
//     let hs = parse(input);

//     let cfg    = Config::new();
//     let ctx    = Context::new(&cfg);
//     let solver = Solver::new(&ctx);

//     let rx  = Int::new_const(&ctx, "rx");
//     let ry  = Int::new_const(&ctx, "ry");
//     let rz  = Int::new_const(&ctx, "rz");
//     let rvx = Int::new_const(&ctx, "rvx");
//     let rvy = Int::new_const(&ctx, "rvy");
//     let rvz = Int::new_const(&ctx, "rvz");

//     // Assumption: After checking the first three hailstones, there is only that one final solution
//     for (_, hs_i) in (0..3).zip(hs.iter().map(|hs| hs.convert_to_z3(&ctx))) {
//         solver.assert(
//             &((rx.clone() - hs_i.x.clone()) * (hs_i.vy.clone() - rvy.clone()))
//                 ._eq(&((ry.clone() - hs_i.y.clone()) * (hs_i.vx.clone() - rvx.clone())))
//         );
//         solver.assert(
//             &((ry.clone() - hs_i.y.clone()) * (hs_i.vz.clone() - rvz.clone()))
//                 ._eq(&((rz.clone() - hs_i.z.clone()) * (hs_i.vy.clone() - rvy.clone())))
//         );
//     }

//     solver.check();
//     let model = solver.get_model().unwrap();
//     let sol = model.eval(&(rx + ry + rz), true).unwrap();

//     return sol.as_u64().unwrap()
// }


// #[cfg(test)]
// mod tests {
//     use indoc::indoc;
//     use super::{solve, solve2};

//     const SAMPLE: &str = indoc! {"19, 13, 30 @ -2,  1, -2
//     18, 19, 22 @ -1, -1, -2
//     20, 25, 34 @ -2, -2, -4
//     12, 31, 28 @ -1, -2, -1
//     20, 19, 15 @  1, -5, -3"};

//     #[test]
//     fn example_part1() {
//         assert_eq!(solve(SAMPLE, true), 2);
//     }

//     #[test]
//     fn example_part2() {
//         assert_eq!(solve2(SAMPLE), 47);
//     }
// }