#include <iostream>
#include <string>
#include <vector>
#include <ranges>
#include <algorithm>
#include <print>
#define MAX_SIZE 100000

using namespace std;

void eraseLetter(
    string& s
    , size_t idx
    , char ch
    , vector<size_t>& pos_vec
) {
    if(s[idx] == ch) {
        s[idx] = ' ';
        if(!pos_vec.empty()) {
            s[pos_vec.back()] = ' ';
            pos_vec.pop_back();
        }
    } else {
        pos_vec.push_back(idx);
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t;
    cin >> t;
    vector<size_t> lower_stack(MAX_SIZE);
    vector<size_t> upper_stack(MAX_SIZE);
    while (t--) {
        string s;
        cin >> s;
        lower_stack.clear();
        upper_stack.clear();
        for (size_t i = 0; i < s.size(); i++) {
            if ((s[i] & 0x20) != 0) {
                eraseLetter(s, i, 'b', lower_stack);
            } else {
                eraseLetter(s, i, 'B', upper_stack);
            }
        }
        println("{}", s | views::split(' ') | views::join | ranges::to<string>());
    }
    return 0;
}
