use common::{ Answer, Solution };

use z3::ast::{Ast, Int};
use z3::{Config, Context, Solver};

pub struct Day24;

impl Solution for Day24 {
    fn name(&self) -> &'static str {
        "Never Tell Me The Odds"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve2(input).into();
    }
}
#[derive(Debug)]
struct HailStone {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    a: f64,
    b: f64,
    c: f64,
}

impl HailStone {
    fn convert_to_z3<'a>(&self, ctx: &'a Context) -> HailStoneZ3<'a> {
        HailStoneZ3 {
            x  : Int::from_i64(ctx, self.x  as i64),
            y  : Int::from_i64(ctx, self.y  as i64),
            z  : Int::from_i64(ctx, self.z  as i64),
            vx : Int::from_i64(ctx, self.vx as i64),
            vy : Int::from_i64(ctx, self.vy as i64),
            vz : Int::from_i64(ctx, self.vz as i64),
        }
    }
}

#[derive(Debug, Clone)]
struct HailStoneZ3<'a> {
    x  : Int<'a>,
    y  : Int<'a>,
    z  : Int<'a>,
    vx : Int<'a>,
    vy : Int<'a>,
    vz : Int<'a>,
}

fn intersection(hs1: &HailStone, hs2: &HailStone, test_limits: bool) -> bool {
    // 1. convert our line definition to this general form
    //      ax + by = c
    //
    // Px = x + t*vx
    // Py = y + t*vy
    //
    // (Px - x) / vx = t = (Pv - y) / vy
    //
    // vy * (Px-x) = vx * (Py-y)
    //
    // vy*Px - vx*Py = vy*x - vx*y
    // >> a = vy, b = -vx, c = vy*x - vx*y
    //
    // 2. now find (x, y) to satisfy the system
    //      a1x + b1y = c1
    //      a2x + b2y = c2
    // while using our a, b, c values for the hailstones

    // Parallel
    if hs1.a * hs2.b == hs1.b * hs2.a { return false }

    // Get intersection
    let x = (hs1.c * hs2.b - hs2.c * hs1.b) /
        (hs1.a * hs2.b - hs1.b * hs2.a);
    let y = (hs2.c * hs1.a - hs1.c * hs2.a) /
        (hs1.a * hs2.b - hs1.b * hs2.a);

    // Check if within boundaries
    let x_min: f64 = if test_limits {7.0} else {200000000000000.0};
    let x_max: f64 = if test_limits {27.0} else {400000000000000.0};
    let y_min = x_min;
    let y_max = x_max;

    if x < x_min || x > x_max || y < y_min || y > y_max { return false }

    // Check if not in future
    if hs1.vx * (x - hs1.x) < 0.0 || hs1.vy * (y - hs1.y) < 0.0 { return false }
    if hs2.vx * (x - hs2.x) < 0.0 || hs2.vy * (y - hs2.y) < 0.0 { return false }

    return true;
}

fn parse(input: &str) -> Vec<HailStone> {
    let mut hs = Vec::<HailStone>::new();

    for line in input.lines() {
        let (pos, vel) = line
            .split_once(" @ ")
            .unwrap();
    
        let p = pos
            .split(",")
            .map(|s| s.trim_start_matches(' ').trim_end_matches(' ').parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let v = vel
        .split(",")
        .map(|s| s.trim_start_matches(' ').trim_end_matches(' ').parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

        let h = HailStone{
            x: p[0] as f64,
            y: p[1] as f64,
            z: p[2] as f64,
            vx: v[0] as f64,
            vy: v[1] as f64,
            vz: v[2] as f64,
            a: v[1] as f64,
            b: -v[0] as f64,
            c: (v[1] * p[0] - v[0] * p[1]) as f64,
        };

        hs.push(h);
    }

    return hs;
}

fn solve(input: &str, test_limits: bool) -> u64 {
    let hs = parse(input);

    let mut ans = 0;
    for (idx, hs1) in hs.iter().enumerate() {
        for hs2 in hs[..idx].iter() {
            if intersection(hs1, hs2, test_limits) { 
                ans += 1;
             }
        }
    }

    return ans;
}

fn solve2(input: &str) -> u64 {
    let hs = parse(input);

    let cfg    = Config::new();
    let ctx    = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let rx  = Int::new_const(&ctx, "rx");
    let ry  = Int::new_const(&ctx, "ry");
    let rz  = Int::new_const(&ctx, "rz");
    let rvx = Int::new_const(&ctx, "rvx");
    let rvy = Int::new_const(&ctx, "rvy");
    let rvz = Int::new_const(&ctx, "rvz");

    // Assumption: After checking the first three hailstones, there is only that one final solution
    for (_, hs_i) in (0..3).zip(hs.iter().map(|hs| hs.convert_to_z3(&ctx))) {
        solver.assert(
            &((rx.clone() - hs_i.x.clone()) * (hs_i.vy.clone() - rvy.clone()))
                ._eq(&((ry.clone() - hs_i.y.clone()) * (hs_i.vx.clone() - rvx.clone())))
        );
        solver.assert(
            &((ry.clone() - hs_i.y.clone()) * (hs_i.vz.clone() - rvz.clone()))
                ._eq(&((rz.clone() - hs_i.z.clone()) * (hs_i.vy.clone() - rvy.clone())))
        );
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let sol = model.eval(&(rx + ry + rz), true).unwrap();

    return sol.as_u64().unwrap()
}


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::{solve, solve2};

    const SAMPLE: &str = indoc! {"19, 13, 30 @ -2,  1, -2
    18, 19, 22 @ -1, -1, -2
    20, 25, 34 @ -2, -2, -4
    12, 31, 28 @ -1, -2, -1
    20, 19, 15 @  1, -5, -3"};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, true), 2);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve2(SAMPLE), 47);
    }
}