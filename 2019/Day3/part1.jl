using DelimitedFiles

# read data for wires
wire1 = readdlm("Day3\\wire1.txt", ',', String, '\n')

# initialize current position at zero zero
current_position = [0, 0]
positions = Int8[]

# loop over command sequences
for i = 1:1:length(wire1)
    println(wire1[i])
    command = wire1[i]
    dir = parse(String,command[1])
    len = parse(Int64,command[2:end])
    # println("Direction: $dir with a length of: $len")
    # println(i)
    # println(dir == "R")

    # # right 
    # if dir == "R"
    #     println("rechts lang")
    #     # current_position = [current_position[1] current_position[2]+len]
    # end
    # # up
    # if dir == "U"
    #     current_position = [current_position[1]+len current_position[2]]
    # end
    # # left 
    # if dir == "L"
    #     current_position = [current_position[1] current_position[2]-len]
    # end
    # # down
    # if dir == "D"
    #     current_position = [current_position[1]-len current_position[2]]
    # end

    # push!(positions, current_position)

    break
end