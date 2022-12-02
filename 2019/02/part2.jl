# "With terminology out of the way, we're ready to proceed. To complete the gravity assist, 
# you need to determine what pair of inputs produces the output 19690720."

# The inputs should still be provided to the program by replacing the values at 
# addresses 1 and 2, just like before. In this program, the value placed in address 1 
# is called the noun, and the value placed in address 2 is called the verb. Each of the 
# two input values will be between 0 and 99, inclusive.

# Once the program has halted, its output is available at address 0, also just like 
# before. Each time you try a pair of inputs, make sure you first reset the computer's 
# memory to the values in the program (your puzzle input) - in other words, don't reuse 
# memory from a previous attempt.

# Find the input noun and verb that cause the program to produce the output 19690720. 
# What is 100 * noun + verb? (For example, if noun=12 and verb=2, the answer would be 1202.)

using DelimitedFiles

flag = false
# get noun and verb of opcode for 19690720 by iterating over all combinations
for noun = 1:1:99
    for verb = 1:1:99
        # always read data new, just using orig data and writing over opcode does not work
        opcode = readdlm("Day2\\opcode.txt", ',', Int, '\n')

        # set iteration steps
        opcode[2] = noun
        opcode[3] = verb

        println("Noun: $noun and Verb: $verb")
        println(opcode)
        # calculate position 0
        for start_pos = 1:4:length(opcode)
            # exit opcode
            if opcode[start_pos] == 99 
                println("Result: $(opcode[1])")
                if opcode[1]== 19690720
                    println("FOUND FOUND FOUND")
                    @goto escape_label
                else
                    break
                end
            # add positions
            elseif opcode[start_pos] == 1
                sum_pos_1 = opcode[start_pos+1]+1
                sum_pos_2 = opcode[start_pos+2]+1
                result_pos = opcode[start_pos+3]+1
                opcode[result_pos] = opcode[sum_pos_1]+opcode[sum_pos_2]
            # multiplicate positions
            elseif opcode[start_pos] == 2
                mult_pos_1 = opcode[start_pos+1]+1
                mult_pos_2 = opcode[start_pos+2]+1
                result_pos = opcode[start_pos+3]+1
                opcode[result_pos] = opcode[mult_pos_1]*opcode[mult_pos_2]
            end
        end
    end
end
@label escape_label
println("Result: $(100*noun+verb)")

# 72 und 64 RESULT