#include <iostream>
#include <ranges>
#include <algorithm>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    string s,t;
    cin >> s >> t;
    cout << (ranges::equal(s, (t | views::reverse)) ? "YES" : "NO");
    return 0;
}
