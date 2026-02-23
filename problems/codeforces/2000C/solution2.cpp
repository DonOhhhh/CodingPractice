#include <iostream>
#include <vector>
#include <string>
#include <cstring>
#include <ext/pb_ds/assoc_container.hpp>

using namespace std;

int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);
    int t;
    cin >> t;
    __gnu_pbds::gp_hash_table<int, int> canonical_int_map;
    int canonical_char_map[26];
    char s[200005];

    while(t--) {
        int n,m;
        cin >> n;
        int next_int_id = 0;
        vector<int> a(n);
        canonical_int_map.clear();
        for(auto& e: a) {
            cin >> e;
            if(canonical_int_map.find(e) == canonical_int_map.end()) {
                canonical_int_map[e] = next_int_id++;
            }
            e = canonical_int_map[e];
        }
        cin >> m;
        while(m--) {
            cin >> s;
            int s_len = strlen(s);
            if(n != s_len) {
                cout << "NO\n";
                continue;
            }
            memset(canonical_char_map, -1, sizeof(canonical_char_map));
            int next_char_id = 0;
            string res = "YES";
            for(int i = 0 ; i < s_len ; i++) {
                size_t idx = s[i] - 'a';
                if(canonical_char_map[idx] == -1) {
                    canonical_char_map[idx] = next_char_id++;
                }
                if(canonical_char_map[idx] != a[i]) {
                    res = "NO";
                    break;
                }
            }
            cout << res << '\n';
        }
    }
}