#include <iostream>
#include <print>
#include <ranges>
#include <unordered_set>
#include <string>
#include <vector>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t;
    cin >> t;
    unordered_set<string_view> myset;
    vector<char> ans;
    while (t--) {
        myset.clear();
        int n; cin >> n;
        vector<string> strs(n, "");
        for (auto& s : strs) {
            cin >> s;
            myset.insert(string_view(s));
        }
        ans.clear();
        for (auto& sv : strs) {
            bool res = false;
            auto sv_len = sv.size();
            for (size_t i = 1; i < sv_len; i++) {
                if (myset.contains(sv.substr(0, i))
                    && myset.contains(sv.substr(i, sv_len - i))) {
                    res = true;
                    break;
                }
            }
            ans.push_back(res ? '1' : '0');
        }
        ans.push_back('\n');
        cout.write(ans.data(), ans.size());
    }
    return 0;
}
