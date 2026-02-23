#include <iostream>
#include <string>
#include <vector>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    string s, t;
    cin >> s >> t;
    size_t s_len = s.size(), t_len = t.size();
    auto same_len = min(s_len, t_len);
    for(size_t i = 1; i <= same_len ; i++) {
        if( s[s_len - i] != t[t_len - i] ) {
            same_len = i - 1;
            break;
        }
    }
    auto res = s_len + t_len - 2 * same_len;
    cout << res;
    return 0;
}
