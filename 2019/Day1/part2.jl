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
    fuel_fuel = 0               # fuel needed for the carried fuel
    mass_fuel = module_fuel     # current mass of fuel 
    while true
        fuel_mass_fuel = floor(mass_fuel/3)-2   # calc fuel needed for current mass of fuel

        # exit loop when fuel for mass of fuel becomes negative
        if fuel_mass_fuel < 0
            break
        end

        # add fuel for mass of fuel to fuel_fuel and set current fuel as new mass of fuel
        mass_fuel = fuel_mass_fuel
        fuel_fuel = fuel_fuel + fuel_mass_fuel
    end

    # total fuel of module
    fuel[index] = module_fuel+fuel_fuel
end

sum_fuel = sum(fuel)
println(sum_fuel)
