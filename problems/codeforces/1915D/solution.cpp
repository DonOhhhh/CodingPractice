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
    string_view sv;
    vector<string_view> output;
    while(t--) {
        int n;
        string s;
        cin >> n >> s;
        sv = string_view(s);
        output.clear();
        while(!sv.empty()) {
            int len = sv.back() == 'a' || sv.back() == 'e' ? 2 : 3;
            output.push_back(sv.substr(sv.size()-len));
            sv.remove_suffix(len);
        }
        cout << ranges::to<string>(output | views::reverse | views::join_with('.')) << '\n';
    }
    return 0;
}
