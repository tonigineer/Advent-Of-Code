#include <fstream>
#include <vector>
#include <string>
#include <sstream>

namespace adoc {


    std::vector<std::string> read_lines_from_file(std::string filename)
    {
        std::string line;
        std::vector<std::string> lines;

        std::ifstream input_file(filename);
        while (std::getline(input_file, line))
        {
            lines.push_back(line);
        };
        return lines;
    };


    std::vector<std::string> split_string(std::string* string_data, std::string delimiter)
    {
        std::vector<std::string> split_list;
        size_t pos = 0;
        std::string token;

        while ((pos = (*string_data).find(delimiter)) != std::string::npos) {
            token = (*string_data).substr(0, pos);
            split_list.push_back(token);
            (*string_data).erase(0, pos + delimiter.length());
        }
        split_list.push_back((*string_data));
        return split_list;
    }


    std::vector<int> convert_string_list_to_integer(std::vector<std::string>* input)
    {
        std::vector<int> output;
        for (auto number = (*input).begin(); number != (*input).end(); number++)
        {
            output.push_back(std::stoi(*number));
        }
        return output;
    }
}
