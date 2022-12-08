using HTTP
using Dates


function main()
    # Determine day and year
    if !isempty(ARGS)
        day = ARGS[1]
        year = ARGS[2]
    else
        day = Dates.day(now())
        year = Dates.year(now())
        month = Dates.month(now())
    end

    if month != 12
        throw(DomainError(
            "There is not Advent of Code this time of the year. \
             Please add commandline argmuments as `julia get_input.jl 1 2016`"
        ))
    end

    # Parse input from web
    io = open("./token", "r")
    cookie = read(io, String)

    cookies = Dict("session"=>cookie)
    r = HTTP.get("https://adventofcode.com/$year/day/$day/input", cookies=cookies)

    # Create file and copy input
    day = lpad(day,2,"0")
    mkpath("./$year/$day")

    open("./$year/$day/$day.in", "w") do io
        write(io, String(r.body))
    end
end


if abspath(PROGRAM_FILE) == @__FILE__
    main()
end