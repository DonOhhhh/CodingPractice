#include <iostream>
#include <string>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t;
    cin >> t;
    while (t--) {
        string s, t;
        cin >> s >> t;
        long long res = t == "a" ? 1
                        : t.find("a") != string::npos
                        ? -1
                        : 1LL << s.size();
        cout << res << "\n";
    }
    return 0;
}
