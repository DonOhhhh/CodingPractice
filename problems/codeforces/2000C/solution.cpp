#include <iostream>
#include <ranges>
#include <string>
#include <vector>
#include <unordered_map>

#define OUTOFRANGE 2e9
#define ALPHABET_CNT 26

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t;
    cin >> t;
    unordered_map<int, char> num_to_char;
    vector<int> char_to_num(ALPHABET_CNT);
    while (t--) {
        int n, m;
        cin >> n;
        vector<int> a(n);
        for (int& e : a) {
            cin >> e;
        }
        cin >> m;
        for(auto _ : views::iota(0, m)) {
            string s;
            cin >> s;
            if(s.size() != a.size()) {
                cout << "NO\n";
                continue;
            }

            num_to_char.clear();
            char_to_num.assign(ALPHABET_CNT, OUTOFRANGE);
            string res = "YES";

            for(const auto& [i, s_ch] : s | views::enumerate) {
                // 문자와 숫자를 비교
                if(char_to_num[s_ch - 'a'] == 2e9) {
                    char_to_num[s_ch - 'a'] = a[i];
                } else {
                    if(char_to_num[s_ch - 'a'] != a[i]) {
                        res = "NO";
                        break;
                    }
                }
                // 숫자와 문자를 비교
                if(!num_to_char[a[i]]) {
                    num_to_char[a[i]] = s_ch;
                } else {
                    if(num_to_char[a[i]] != s_ch) {
                        res = "NO";
                        break;
                    }
                }
            }
            cout << res << '\n';
        }
    }
    return 0;
}
