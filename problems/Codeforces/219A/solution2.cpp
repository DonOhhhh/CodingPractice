#include <iostream>
#include <string>
#include <vector>
#include <ranges>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int k;
    string s;
    cin >> k >> s;
    vector<size_t> count(26,0);
    for(char& c : s) {
        count[c-'a']++;
    }
    vector<char> pattern;
    pattern.reserve(s.size() / k);
    for(auto const [idx, n]: count | views::enumerate) {
        if(n % k != 0) {
            cout << "-1\n";
            return 0;
        }
        for(size_t i = 0 ; i < (n / k) ; i++) {
            pattern.push_back(static_cast<char>('a' + idx));
        }
    }
    for(auto _ : views::iota(0, k))
        cout.write(pattern.data(), pattern.size());
    cout << '\n';
    return 0;
}