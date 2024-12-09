TODAY := $(shell TZ=Europe/Berlin date +%y%m%d)
YEAR ?= $(shell TZ=Europe/Berlin date +%Y)

POSSIBLE_DAYS := 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25
PUZZLE_DAY := $(or $(filter $(TODAY:2412%=%), $(POSSIBLE_DAYS)), help)

.PHONY: $(POSSIBLE_DAYS) all help test solve default clean

default: $(PUZZLE_DAY)

$(POSSIBLE_DAYS): %:
	cargo test -p aoc_${YEAR} day_$*::tests::example_part
	cargo run --release solve ${YEAR} $*

test:
	cargo test -p aoc_2024 day_$(PUZZLE_DAY)::tests::example_part

solve:
	cargo run --release solve ${YEAR} ${PUZZLE_DAY}

all:
	cargo test -p aoc_${YEAR}
	cargo run --release list ${YEAR}

help:
	@echo "Usage: make [TARGET] [YEAR=<year>]"
	@echo
	@echo "By default, during December, running 'make' without arguments"
	@echo "will attempt to run the current day's tests and puzzle solution."
	@echo
	@echo "Targets:"
	@echo "  {01..25}   Run the solution and example tests for a specific puzzle day."
	@echo "             Example: 'make 01' runs the Day 01 solution."
	@echo
	@echo "  test       Run the test examples for today's puzzle."
	@echo "  solve      Run the solution for today's puzzle."
	@echo "  all        Run all available days' solutions and tests for this year."
	@echo "  help       Show this help message."
	@echo
	@echo "By default, the Makefile uses the current year. If you need to use a different year,"
	@echo "specify it on the command line, e.g.:"
	@echo "  make YEAR=2023"

clean:
	cargo clean
