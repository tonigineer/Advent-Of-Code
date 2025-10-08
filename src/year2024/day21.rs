//! Keypad Conundrum
//!
//! Precompute the shortest way for all keypad combinations. For
//! both pads.

use crate::common::parse::*;
use crate::common::position::*;
use ahash::AHashMap;
use std::iter::{once, repeat_n};

type KeyCombiPaths = AHashMap<(char, char), Vec<String>>;
type Cache = AHashMap<(char, char, usize), usize>;
type ParsedInput = (Vec<String>, Vec<usize>, KeyCombiPaths);

pub fn parse(input: &str) -> ParsedInput {
    let input = input.trim();

    let code_strings = input.lines().map(String::from).collect::<Vec<_>>();
    let code_numeric = input.lines().map(|c| c.parse_uint::<usize>()).collect::<Vec<_>>();

    let key_combi_paths = process_keypads();
    (code_strings, code_numeric, key_combi_paths)
}

pub fn part1(input: &ParsedInput) -> usize {
    let num_keypads = 3;
    solve(input, num_keypads)
}

pub fn part2(input: &ParsedInput) -> usize {
    let num_keypads = 26;
    solve(input, num_keypads)
}

fn solve(input: &ParsedInput, num_keypads: usize) -> usize {
    let (codes, nums, paths) = input;
    let mut cache: Cache = AHashMap::new();

    codes.iter().zip(nums).map(|(c, n)| dfs(&mut cache, paths, c, num_keypads) * n).sum()
}

fn dfs(cache: &mut Cache, key_combi_paths: &KeyCombiPaths, code: &str, depth: usize) -> usize {
    // Depth is basically number of keypads chained.
    if depth == 0 {
        return code.len();
    }

    // Initial or final position is alsways A
    let mut previous = 'A';
    let mut total_presses = 0;

    for current in code.chars() {
        let key = (previous, current, depth);

        let additional_presses = if let Some(&presses) = cache.get(&key) {
            presses
        } else {
            let presses = key_combi_paths[&(previous, current)]
                .iter()
                .map(|next| dfs(cache, key_combi_paths, next, depth - 1))
                .min()
                .unwrap();
            cache.insert(key, presses);
            presses
        };

        total_presses += additional_presses;
        previous = current;
    }

    total_presses
}

fn process_keypads() -> KeyCombiPaths {
    // Coordinates based on top left corner (7)
    let numeric_gap = Position::new(0, 3);
    let numeric_keys = [
        ('7', Position::new(0, 0)),
        ('8', Position::new(1, 0)),
        ('9', Position::new(2, 0)),
        ('4', Position::new(0, 1)),
        ('5', Position::new(1, 1)),
        ('6', Position::new(2, 1)),
        ('1', Position::new(0, 2)),
        ('2', Position::new(1, 2)),
        ('3', Position::new(2, 2)),
        ('0', Position::new(1, 3)),
        ('A', Position::new(2, 3)),
    ];

    let directional_gap = Position::new(0, 0);
    let directional_keys = [
        ('^', Position::new(1, 0)),
        ('A', Position::new(2, 0)),
        ('<', Position::new(0, 1)),
        ('v', Position::new(1, 1)),
        ('>', Position::new(2, 1)),
    ];

    let mut key_combi_paths = AHashMap::new();
    find_keypad_paths(&mut key_combi_paths, &numeric_keys, numeric_gap);
    find_keypad_paths(&mut key_combi_paths, &directional_keys, directional_gap);

    key_combi_paths
}

fn find_keypad_paths(
    key_combi_paths: &mut KeyCombiPaths,
    keys: &[(char, Position)],
    gap: Position,
) {
    for &(key_from, pos_from) in keys {
        for &(key_to, pos_to) in keys {
            let horizontal = || {
                let times = pos_from.x.abs_diff(pos_to.x) as usize;
                let dir = if pos_from.x > pos_to.x { '<' } else { '>' };
                repeat_n(dir, times)
            };

            let vertical = || {
                let times = pos_from.y.abs_diff(pos_to.y) as usize;
                let dir = if pos_from.y > pos_to.y { '^' } else { 'v' };
                repeat_n(dir, times)
            };

            if Position::new(pos_from.x, pos_to.y) != gap {
                let path = vertical().chain(horizontal()).chain(once('A')).collect();
                key_combi_paths.entry((key_from, key_to)).or_default().push(path);
            }

            // Skip adding path twice, if there are directly above/under or left/right
            if Position::new(pos_to.x, pos_from.y) != gap
                && pos_from.x != pos_to.x
                && pos_from.y != pos_to.y
            {
                let path = horizontal().chain(vertical()).chain(once('A')).collect();
                key_combi_paths.entry((key_from, key_to)).or_default().push(path);
            }
        }
    }
}
