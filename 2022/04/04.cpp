#include <iostream>
#include <algorithm>
#include <fstream>


std::ifstream f("04.in");

int s1, e1;
int s2, e2;
char a;

int ans = 0;
int ans2 = 0;

int main() {
    while (true) {
        f >> s1 >> a >> e1 >> a >> s2 >> a >> e2;

        if ((s1 <= s2 && e2 <= e1) || (s2 <= s1 && e1 <= e2)) ans++;
        if (std::max(s1, s2) <= std::min(e1, e2)) ans2++;

        if (f.eof()) break;
    }
    
    std::cout << "1. Solution: " << ans << std::endl;
    std::cout << "2. Solution: " << ans2 << std::endl;
}