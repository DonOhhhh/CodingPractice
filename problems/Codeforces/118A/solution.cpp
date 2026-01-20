#include <cctype>
#include <iostream>
#include <iterator>
#include <vector>

using namespace std;

int main() {
    vector<char> s{istreambuf_iterator<char>{cin}, {}};

    uint32_t check = 0 | (1 << 0) | (1 << 4) | (1 << 8) | (1 << 14) | (1 << 20) | (1 << 24);
    vector<char> output;
    output.reserve(1 << 7);
    for(char& c : s) {
        if(isspace(c)) continue;
        char lower_c = c | 0x20;
        if(((check >> (lower_c - 'a')) & 1) == 1) continue;
        output.push_back('.');
        output.push_back(lower_c);
    }

    cout.write(output.data(), output.size());
    return 0;
}
