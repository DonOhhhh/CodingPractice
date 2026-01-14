#include <iostream> // cin, cout
#include <iterator> // istreambuf_iterator
#include <vector>   // vector
#include <ranges>   // views
#include <print>    // print
#define ALPHA_NUM 26

int main() {
    using namespace std;
    string s{istreambuf_iterator<char>{cin}, {}};
    vector<int> count(ALPHA_NUM, 0);

    for(auto c : s | views::filter([](char x) { return x >= 'a' && x <= 'z'; })) {
        count[c - 'a']++;
    }

    for (auto [idx, c] : count | views::enumerate) {
        print("{}{}", c, (idx == 25 ? "" : " "));
    }
    return 0;
}