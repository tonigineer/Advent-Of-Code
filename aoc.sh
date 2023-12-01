#!/usr/bin/env bash

cd ~/Engineering/Competetive-Programming/Advent-Of-Code || exit 1

function read_cookie() {
	export AOC_COOKIE=$(cat ~/.config/aocd/token)

	if [ "$AOC_COOKIE" = "" ]; then
		echo "Could not read cookie. Please put into ~/.config/aocd/token"
		exit 1
	fi
}

function create_template_rust() {
	mkdir -p ./${1}/rust-project/inputs ./${1}/rust-project/src/bin

	echo \
		"
fn solve(input: &str, part2: bool) -> i32 {
    return 0;
}

fn main() {
    let input = include_str!(\"../../inputs/$(printf %02d $2).in\").trim();

    print!(\"󰎤 {} \t\", solve(&input, false));
    print!(\"󰎧 {} \",   solve(&input, true));
}" >"${1}"/rust-project/src/bin/"$(printf "%02d" "$2")".rs
}

case $1 in
--show-cookie)
	read_cookie
	echo "$AOC_COOKIE"
	;;
--get-input)
	[ "$2" ] && year="$2" || year=$(date +%Y)
	[ "$3" ] && day="$3" || day=$(date +%-d)

	read_cookie
	create_template_rust "$year" "$day"

	curl --cookie "session=$AOC_COOKIE" https://adventofcode.com/"$year"/day/"$day"/input >"${year}"/rust-project/inputs/"$(printf "%02d" "$day")".in
	;;
--update-badges)
	source .venv/bin/activate
	python3 tools/aoc_tiles/create_aoc_tiles.py
	rm -rf tools/aoc_tiles/.aoc_tiles_cache
	;;
*)
	echo "
* ARGUMENTS
    aoc --show-cookie
    aoc --get-input 2017 13
    aoc --get-input
    aoc --update-badges

* NOTE
    1. no date check implemented
    2. .rs file created in hardcoded structure
"
	;;
esac
