#include <fstream>
#include <vector>
#include <string>


std::vector<std::string> read_lines_from_file(std::string filename)
{
    std::string line;
    std::vector<std::string> lines;

    std::ifstream input_file(filename);
    while (input_file >> line)
    {
        lines.push_back(line);
    };
    return lines;
}

std::vector<int> convert_strings_to_integer(std::vector<std::string> lines)
{
    std::vector<int> numbers;
    for (auto number = lines.begin(); number != lines.end(); ++number)
    {
        numbers.push_back(std::stoi(*number));
    }
    return numbers;
}
