#include <algorithm>
#include <iostream>
#include <print>
#include <string>
#include <vector>

int main() {
    using namespace std;
    int t;
    cin >> t;
    while (t--) {
        int _n, k;
        string s;
        cin >> _n >> k >> s;
        vector<int> odd_count(26, 0);
        for (char c : s) {
            odd_count[c - 'a']++;
        }
        println("{}",
                k + 1 >= ranges::count_if(
                    odd_count,
                    [](auto&& c) { return c % 2 == 1; })

                    ? "YES"
                    : "NO");
    }
    return 0;
}
