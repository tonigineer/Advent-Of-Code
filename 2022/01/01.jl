# https://adventofcode.com/2022/day/1

io = open("./2022/01/01.in", "r")
input = read(io, String)

elves = split(input, "\n\n")
elves = split.(elves)

total_calories = [sum(parse.(Int, elf)) for elf in elves]
sort!(total_calories, rev=true)

println("🎄 1. Solution: $(total_calories[1])")
println("🎅 2. Solution: $(sum(total_calories[1:3]))")
