#include <iostream>
#include <string>
#include <vector>
#include <cmath>
#include <algorithm>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t; cin >> t;
    while(t--) {
        int n,k;
        cin >> n >> k;
        string s;
        cin >> s;
        int ans = 0;
        vector<int> cnt(128,0);
        for(char& ch: s) {
            cnt[ch]++;
        }
        for(size_t i = 0; i < 26 ; i++) {
            ans += min(cnt['a'+i], cnt['A'+i]);
            int possible_unpaired = abs(cnt['a'+i] - cnt['A'+i]) / 2;
            int added = min(possible_unpaired, k);
            ans += added;
            k -= added;
        }
        cout << ans << '\n';
    }
    return 0;
}
