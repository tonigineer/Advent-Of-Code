TZ    ?= Europe/Berlin
YEAR  ?= $(shell TZ=$(TZ) date +%Y)
DAY   ?= $(shell TZ=$(TZ) date +%d)

POSSIBLE_DAYS := $(shell seq -w 1 25)


ifeq ($(filter $(DAY),$(POSSIBLE_DAYS)),$(DAY))
.DEFAULT_GOAL := $(DAY)
else
.DEFAULT_GOAL := help
endif


SHELL := bash
.SHELLFLAGS := -eu -o pipefail -c
.ONESHELL:
Q ?= @
MAKEFLAGS += --no-builtin-rules


tests/year$(YEAR)/day%.rs:
	$(Q)if [[ -e "$@" ]]; then
		echo "Skip $@ (exists)";
	else
		mkdir -p "$(dir $@)";
		printf "use aoc::year${YEAR}::day$*::*;\n\
\n\
const EXAMPLE: &str = \"\";\n\
\n\
#[test]\n\
fn part1_test() {\n\
    let input = parse(EXAMPLE);\n\
    assert_eq!(part1(&input), 0);\n\
}\n\
\n\
#[test]\n\
fn part2_test() {\n\
    let input = parse(EXAMPLE);\n\
    assert_eq!(part2(&input), 0);\n\
}\n" > "$@";
		echo "Created $@";
	fi

src/year$(YEAR)/day%.rs:
	$(Q)if [[ -e "$@" ]]; then
		echo "Skip $@ (exists)";
	else
		mkdir -p "$(dir $@)";
		printf "//! # FIX: Title\n\
//!\n\
//! FIX: Summary line.\n\
\n\
pub fn parse(input: &str) -> &str {\n\
    input\n\
}\n\
\n\
pub fn part1(input: &str) -> u32 {\n\
    13\n\
}\n\
\n\
pub fn part2(input: &str) -> u32 {\n\
    13\n\
}\n" > "$@";
		echo "Created $@";
	fi


$(POSSIBLE_DAYS): %:
	$(Q)$(MAKE) tests/year$(YEAR)/day$@.rs
	$(Q)$(MAKE) src/year$(YEAR)/day$@.rs

	$(Q)cargo build --release
	$(Q)cargo test year$(YEAR)::day$@ -- --nocapture
	$(Q)cargo run --release year$(YEAR)::day$@

new:
	$(Q)$(MAKE) tests/year$(YEAR)/day$(DAY).rs
	$(Q)$(MAKE) src/year$(YEAR)/day$(DAY).rs

test:
	$(Q)cargo test year$(YEAR)::day$(DAY) -- --nocapture

solve:
	$(Q)cargo run --release year$(YEAR)::day$(DAY)

year:
	$(Q)cargo build --release
	$(Q)cargo test year$(YEAR)
	$(Q)cargo run --release year$(YEAR)

all:
	$(Q)cargo build --release
	$(Q)cargo test
	$(Q)cargo run --release

bench:
	$(Q)cargo bench -- year$(YEAR)

clean:
	$(Q)cargo clean

checks:
	$(Q)cargo fmt --check -- `find . -name "*.rs"`
	$(Q)cargo clippy --all-targets --all-features -- --deny warnings

help:
	@echo "Advent of Code — Make targets"
	@echo
	@echo "USAGE"
	@echo "  make [TARGET] [YEAR=<yyyy>] [DAY=<dd>] [TZ=<Area/City>] [Q=]"
	@echo
	@echo "DEFAULTS"
	@echo "  DAY and YEAR are derived from the current local date (TZ=$(TZ))."
	@echo "  These defaults apply to ALL targets below unless you override them."
	@echo
	@echo "OVERRIDES"
	@echo "  make DAY=07"
	@echo "  make YEAR=2023"
	@echo "  make DAY=14 YEAR=2024"
	@echo
	@echo "TARGETS"
	@echo "  {01..25}   Run tests and the solution for that day using the selected YEAR."
	@echo "  new        Create only the files for DAY/YEAR (no build/run)."
	@echo "  test       Run tests for DAY/YEAR."
	@echo "  solve      Run the solution for DAY/YEAR."
	@echo "  bench      Run Criterion benchmarks for YEAR."
	@echo "  clean      Remove build artifacts."
	@echo "  checks     Run clippy and fmt with cargo"
	@echo "  help       Show this help."
	@echo
	@echo "EXAMPLES"
	@echo "  make                            # uses today's DAY and YEAR (if 01..25), else help"
	@echo "  make 03                         # solve+test Day 03 for today's YEAR"
	@echo "  make test DAY=05                # tests for Day 05 (current YEAR)"
	@echo "  make solve YEAR=2023 DAY=14     # solve for Day 14 - Year 2023"
	@echo "  make bench YEAR=2024            # run benches for 2024"
	@echo

.PHONY: $(POSSIBLE_DAYS) bench checks clean help new solve test

