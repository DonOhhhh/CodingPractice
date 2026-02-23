#include <iostream>
#include <string>
#include <vector>
#include <ranges>
#include <algorithm>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int k;
    string s;
    cin >> k >> s;
    vector<size_t> count(26, 0);
    for(char& c: s) {
        count[c - 'a']++;
    }
    auto res = ranges::all_of(count.begin(), count.end(), [k](auto&& n){return n % k == 0;});
    if( !res ){
        cout << "-1\n";
    } else {
        string pattern = 
            count 
            | views::enumerate 
            | views::transform([k](auto&& pair) {
                auto [i,n] = pair;
                return 
                    n == 0 
                    ? string()
                    : string(n / k, static_cast<char>('a' + i));
            }) 
            | views::join 
            | ranges::to<string>();
        cout << (views::repeat(pattern, k) | views::join | ranges::to<string>()) << '\n';
    }
    return 0;
}
