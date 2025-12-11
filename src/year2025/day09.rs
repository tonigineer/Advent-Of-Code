//! # Movie Theater
//!
//!

use crate::common::parse::ParseInteger;

use itertools::Itertools;
use std::collections::HashMap;

pub fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut it = l.parse_int_iter();
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect()
}

fn point_within_polygon(
    cache: &mut HashMap<(i64, i64), bool>,
    tiles: &Vec<(i64, i64)>,
    point: (i64, i64),
) -> bool {
    if let Some(&b) = cache.get(&point) {
        return b;
    }

    cache.insert(point, true);

    let (x, y) = point;
    let mut within = false;

    for (&(x1, y1), &(x2, y2)) in tiles.iter().zip(tiles.iter().skip(1).chain(tiles.iter().take(1)))
    {
        // Point is on an edge
        if x == x1 && x == x2 && y1.min(y2) <= y && y <= y1.max(y2)
            || y == y1 && y == y2 && x1.min(x2) <= x && x <= x1.max(x2)
        {
            return true;
        }

        // Ray finding
        if ((y1 > y) != (y2 > y)) && (x < (x2 - x1) * (y - y1) / (y2 - y1) + x1) {
            within = !within;
        }
    }

    cache.entry(point).and_modify(|p| *p = within);
    within
}

fn edge_intersects_rect(edge: ((i64, i64), (i64, i64)), rect: ((i64, i64), (i64, i64))) -> bool {
    let (x1, y1) = edge.0;
    let (x2, y2) = edge.1;
    let (rx1, ry1) = rect.0;
    let (rx2, ry2) = rect.1;

    if y1 == y2 && (ry1 < y1 && y1 < ry2) && (x1.max(x2) > rx1 && x1.min(x2) < rx2) {
        return true;
    } else if (rx1 < x1 && x1 < rx2) && (y1.max(y2) > ry1 && y1.min(y2) < ry2) {
        return true;
    }

    false
}

fn is_valid(
    cache: &mut HashMap<(i64, i64), bool>,
    tiles: &Vec<(i64, i64)>,
    p1: &(i64, i64),
    p2: &(i64, i64),
) -> bool {
    // NOTE: sorting is important for `edge_intersects_rect` function! Rectangle must span from top left to
    // bottom right.
    let mut xs = [p1.0, p2.0];
    let mut ys = [p1.1, p2.1];

    xs.sort();
    ys.sort();

    let top_left = (xs[0], ys[0]);
    let bottom_left = (xs[0], ys[1]);
    let top_right = (xs[1], ys[0]);
    let bottom_right = (xs[1], ys[1]);

    let corners = [top_left, bottom_left, top_right, bottom_right];

    if !corners.iter().all(|&p| point_within_polygon(cache, tiles, p)) {
        return false;
    }

    for (&e1, &e2) in tiles.iter().zip(tiles.iter().skip(1).chain(tiles.iter().take(1))) {
        if edge_intersects_rect((e1, e2), (top_left, bottom_right)) {
            return false;
        }
    }

    true
}

pub fn part1(tiles: &Vec<(i64, i64)>) -> u64 {
    tiles
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1))
        .max()
        .unwrap()
}

pub fn part2(tiles: &Vec<(i64, i64)>) -> u64 {
    let mut cache = HashMap::new();

    tiles.iter().tuple_combinations().fold(0, |mut acc, x: (&(i64, i64), &(i64, i64))| {
        let p1 = x.0;
        let p2 = x.1;

        let size = (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1);

        if size > acc && is_valid(&mut cache, &tiles, &p1, &p2) {
            acc = size;
        }

        acc
    })
}
