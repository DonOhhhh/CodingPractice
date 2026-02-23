#include <iostream>
#include <string>
#include <ranges>
#include <algorithm>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    string s;
    cin >> s;

    auto tail = s | views::drop(1);
    bool condition = ranges::all_of(tail, [](char c) {
        return c >= 'A' && c <= 'Z';
    });

    if (condition) {
        for (char& c : s) {
            c ^= 32;
        }
    }

    cout << s;
    return 0;
}