# Once you have a working computer, the first step is to restore the gravity 
# assist program (your puzzle input) to the "1202 program alarm" state it had 
# just before the last computer caught fire. To do this, before running the 
# program, replace position 1 with the value 12 and replace position 2 with 
# the value 2. What value is left at position 0 after the program halts?

using DelimitedFiles

opcode = readdlm("Day2\\opcode.txt", ',', Int, '\n')


# 1202 program alarm from task
opcode[2] = 12
opcode[3] = 2

# calculate position 0
for start_pos = 1:4:length(opcode)
    println(opcode)
    # println(start_pos)
    # println(opcode[start_pos])

    # exit opcode
    if opcode[start_pos] == 99 
        println("Result: $(opcode[1])")
        break
    # add positions
    elseif opcode[start_pos] == 1
        opcode[opcode[start_pos+3]+1] = opcode[opcode[start_pos+1]+1]+opcode[opcode[start_pos+2]+1]
    # multiplicate positions
    elseif opcode[start_pos] == 2
        opcode[opcode[start_pos+3]+1] = opcode[opcode[start_pos+1]+1]*opcode[opcode[start_pos+2]+1]
    end
end