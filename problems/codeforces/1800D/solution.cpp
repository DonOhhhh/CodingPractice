#include <iostream>
#include <string>
#include <vector>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t; cin >> t;
    while(t--) {
        int n; cin >> n;
        string s; cin >> s;
        size_t s_len = s.size();
        size_t ans = s_len - 1;
        for(size_t i = 0; i < s_len - 2; i++) {
            if(s[i] == s[i+2]) ans--;
        }
        cout << ans << '\n';
    }
    return 0;
}
