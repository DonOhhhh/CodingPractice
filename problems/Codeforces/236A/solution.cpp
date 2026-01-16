#include <iostream>
#include <string>
// #include <bit>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    string s;
    cin >> s;

    int mask = 0;

    for (char c : s) {
        mask |= (1 << (c - 'a'));
    }

    // C++20부터는 <bit> 헤더의 std::popcount(mask)를 써도 됩니다.
    // 단, popcount는 unsigned int만 받음
    if (__builtin_popcount(mask) % 2 == 0) {
    // if(popcount(mask) % 2 == 0) {
        cout << "CHAT WITH HER!";
    } else {
        cout << "IGNORE HIM!";
    }

    return 0;
}