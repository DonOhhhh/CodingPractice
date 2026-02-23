#include <iostream>
#include <string>
#include <vector>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    string s;
    cin >> s;
    size_t n = s.size();
    size_t cnt = 0;
    for (size_t i = 0; i < n / 2; i++) {
        if (s[i] != s[n - 1 - i]) {
            cnt++;
        }
    }
    cout << (cnt == 1 || (cnt == 0 && n % 2 == 1) ? "YES" : "NO");
    return 0;
}
