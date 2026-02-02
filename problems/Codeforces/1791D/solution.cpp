#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t;
    cin >> t;
    while(t--) {
        int _;
        string s;
        cin >> _ >> s;
        vector<int> left_freq(26, 0);
        vector<int> right_freq(26, 0);
        uint32_t left_distinct = 0;
        uint32_t right_distinct = 0;
        uint32_t max_cnt = 0;
        for(char& c: s) {
            int idx = static_cast<int>(c - 'a');
            if (right_freq[idx]++ == 0) {
                right_distinct++;
            }
        }

        for(char& c: s) {
            int idx = static_cast<int>(c - 'a');
            if(left_freq[idx]++ == 0) {
                left_distinct++;
            }
            if(--right_freq[idx] == 0) {
                right_distinct--;
            }
            max_cnt = max(max_cnt, left_distinct+right_distinct);
        }
        cout << max_cnt << '\n';
    }
    return 0;
}
