#include <iostream>
#include <string>
#include <vector>
#include <ranges>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t;
    cin >> t;
    while(t--) {
        int n;
        string s;
        cin >> n >> s;
        vector<char> cnt(26,0);
        int sum = 0;
        for(auto const [i, ch] : views::enumerate(s)) {
            size_t idx = ch - 'a';
            if(cnt[idx] != 0) continue;
            sum += n - i;
            cnt[idx] = 1;
        }
        cout << sum << '\n';
    }
    return 0;
}
