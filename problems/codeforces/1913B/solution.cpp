#include <iostream>
#include <string>
#include <vector>

using namespace std;
void solve() {
    string s;
    cin >> s;
    int zeros = 0, ones = 0;
    for (char c : s) {
        c == '0' ? zeros++ : ones++;
    }

    vector<char> output;
    int len = s.length();
    for (int i = 0; i < len; i++) {
        if (s[i] == '0') {
            if (ones > 0) {
                ones--;
            } else {
                cout << len - i << '\n';
                return;
            }
        } else {
            if(zeros > 0) {
                zeros--;
            } else {
                cout << len - i << '\n';
                return;
            }
        }
    }
    cout << '0' << '\n';
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t;
    cin >> t;
    while (t--) {
        solve();
    }
    return 0;
}
