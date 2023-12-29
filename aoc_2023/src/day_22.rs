use common::{ Answer, Solution };
use std::collections::{ HashMap, HashSet, VecDeque };

pub struct Day22;

impl Solution for Day22 {
    fn name(&self) -> &'static str {
        "Sand Slabs"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn overlap(a: &[i64; 6], b: &[i64; 6]) -> bool {
    return a[0].max(b[0]) <= a[3].min(b[3]) && a[1].max(b[1]) <= a[4].min(b[4]);
}

fn falling(mut bricks: Vec<[i64; 6]>) -> Vec<[i64; 6]> {
    let mut bricks_processed = Vec::<[i64; 6]>::new();
    for (idx, brick) in bricks.iter_mut().enumerate() {
        let mut z_max = 1;
        for next_brick in bricks_processed.iter().take(idx) {
            if overlap(brick, next_brick) {
                z_max = z_max.max(next_brick[5] + 1);
            }
        }
        brick[5] -= brick[2] - z_max;
        brick[2] = z_max;
        bricks_processed.push(brick.clone())
    }
    bricks.sort_by_key(|b| b[2]);
    return bricks;

}

fn solve(input: &str, part2: bool) -> u64 {
    let mut bricks = Vec::<[i64; 6]>::new();
    for line in input.lines() {
        let brick: [i64; 6] = line
            .replace('~', ",")
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>().as_slice().try_into().unwrap();
        bricks.push(brick);
    }

    bricks.sort_by_key(|b| b[2]);
    bricks = falling(bricks);

    let mut l_supports_u = HashMap::<usize,HashSet<usize>>::new();
    let mut u_supports_l = HashMap::<usize,HashSet<usize>>::new();
    for (u, upper) in bricks.iter().enumerate() {
        for (l, lower) in bricks.iter().take(u).enumerate() {
            if overlap(lower, upper) && lower[5] + 1 == upper[2] {
                l_supports_u
                    .entry(l)
                    .and_modify(|m| { m.insert(u); })
                    .or_insert( HashSet::from_iter([u]) );
                u_supports_l
                    .entry(u)
                    .and_modify(|m| { m.insert(l); })
                    .or_insert( HashSet::from_iter([l]) );
            }
        }
    }

    let mut ans = 0;
    if !part2 {

        for idx in 0..bricks.len() {
            if !l_supports_u.contains_key(&idx) { ans += 1; continue }
        
            let mut all = true;
            for u in l_supports_u[&idx].iter() {
                if u_supports_l[u].len() < 2 {
                    all = false;
                }
            }

            if all { ans += 1 }
        }
        return ans;
    }

    for idx in 0..bricks.len() {
        let mut q = VecDeque::<usize>::new();
        let mut falling = HashSet::<usize>::new();
        falling.insert(idx);

        if !l_supports_u.contains_key(&idx) { continue }

        for u in l_supports_u[&idx].iter() {
            if !u_supports_l.contains_key(&u) { continue }

            if u_supports_l[&u].len() == 1 {
                q.push_back(*u);
                falling.insert(*u);
            }
        }

        while !q.is_empty() {
            let l = q.pop_front().unwrap();
            if !l_supports_u.contains_key(&l) { continue }

            for l in l_supports_u[&l].iter() {
                if falling.contains(&l) { continue }

                let mut subset = true;
                for u in u_supports_l[&l].iter() {
                    if !falling.contains(u) { subset = false }
                }
                if subset {
                    q.push_back(*l);
                    falling.insert(*l);
                }
            }
        }
        ans += (falling.len() - 1) as u64;
    }
    return ans as u64;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"1,0,1~1,2,1
    0,0,2~2,0,2
    0,2,3~2,2,3
    0,0,4~0,2,4
    2,0,5~2,2,5
    0,1,6~2,1,6
    1,1,8~1,1,9"};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 5);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 7);
    }
}