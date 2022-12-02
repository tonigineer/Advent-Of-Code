#include <iostream>
#include <vector>
#include <algorithm>

#include "adoc.h"

#define INPUT_FILENAME "data/7.in"


void part1(std::vector<int> positions)
{
    std::sort(positions.begin(), positions.end());
    int median = positions.at(positions.size() / 2);

    int fuel = 0;
    for (auto p = positions.begin(); p != positions.end(); p++)
    {
        fuel += abs(median - *p);
    }
    std::cout << "Part 1: " << fuel << std::endl;
}

void part2(std::vector<int> positions)
{
    std::vector<int> fuel_spent;
    std::sort(positions.begin(), positions.end());

    for (auto target = positions.begin(); target != positions.end(); target++)
    {
        int fuel = 0;
        for (auto p = positions.begin(); p != positions.end(); p++) 
        {
            int diff = abs(*target - *p);
            fuel += (diff*(diff+1))/2;
        }
        fuel_spent.push_back(fuel);
    }

    auto result = *std::min_element(fuel_spent.begin(), fuel_spent.end());
    std::cout << "Part 2: " << result << std::endl;
}


int main() 
{
    std::vector<std::string> lines = adoc::read_lines_from_file(INPUT_FILENAME);
    std::vector<std::string> integer_strings = adoc::split_string(&lines.at(0), ",");
    std::vector<int> positions = adoc::convert_string_list_to_integer(&integer_strings);

    part1(positions);
    part2(positions);
}
