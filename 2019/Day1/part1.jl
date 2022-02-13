# What is the sum of the fuel requirements for all of the modules 
# on your spacecraft when also taking into account the mass of 
# the added fuel? (Calculate the fuel requirements for each 
# module separately, then add them all up at the end.)

using DelimitedFiles

# read input file
mass = readdlm("Day1\\modules_mass.txt", '\t', Int, '\n')
fuel = zeros(Int32, 100)

for (index, value) in enumerate(mass)
    # calc fuel for module mass
    module_fuel = floor(mass[index]/3)-2
    # calc fuel for fuel
    fuel_fuel = 0

    # total fuel of module
    fuel[index] = module_fuel+fuel_fuel
end

sum_fuel = sum(fuel)
println(sum_fuel)
