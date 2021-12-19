#include <iostream>
#include <vector>

#include "adoc.h"

#define INPUT_FILENAME "data/1.in"


int part1(std::vector<int>* depths)
{
    int counter = 0;
    for (auto depth = (*depths).begin() + 1; depth != (*depths).end(); ++depth)
    {
        if (*depth > *(depth - 1)) ++counter;
    }

    std::cout << "Solution part 1: " << counter << std::endl;
    return 0;
}


int part2(std::vector<int>* depths)
{
    std::vector<int> sliding_windows;
    for (auto depth = (*depths).begin() + 2; depth != (*depths).end(); ++depth)
    {
        sliding_windows.push_back(*depth + *(depth-1) + *(depth-2));
    }

    int counter = 0;
    for (auto window = sliding_windows.begin() + 1; window != sliding_windows.end(); ++window)
    {
        if (*window > *(window - 1)) ++counter;
    }

    std::cout << "Solution part 2: " << counter << std::endl;
    return 0;
}


int main()
{
    std::vector<std::string> lines = adoc::read_lines_from_file(INPUT_FILENAME);
    std::vector<int> depths = adoc::convert_string_list_to_integer(&lines);

    part1(&depths);
    part2(&depths);
}
