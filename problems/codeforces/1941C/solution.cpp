#include <iostream>
#include <string>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t;
    cin >> t;
    while (t--) {
        int n;
        string s;
        cin >> n >> s;
        string_view sv(s);
        int ans = 0;
        int i = 0;
        while(i < n) {
            if (i + 4 < n && sv.substr(i, 5) == "mapie") {
                ans++;
                i += 5;
            } else if (i + 2 < n
                       && (sv.substr(i, 3) == "map"
                           || sv.substr(i, 3) == "pie")) {
                ans++;
                i += 3;
            } else {
                i++;
            }
        }
        cout << ans << '\n';
    }
    return 0;
}
