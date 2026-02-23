#include <iostream>
#include <string>
#include <vector>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t;
    cin >> t;
    while (t--) {
        size_t n;
        cin >> n;
        string s;
        cin >> s;
        size_t leading_zeros = 0;
        while (leading_zeros < n
               && s[leading_zeros] == '0') {
            leading_zeros++;
        }

        size_t trailing_ones = 0;
        while (trailing_ones < n
               && s[n - 1 - trailing_ones] == '1') {
            trailing_ones++;
        }

        for (size_t i = 0; i < leading_zeros; i++) {
            cout << "0";
        }

        if (leading_zeros + trailing_ones < n) {
            cout << "0";
        }

        for (size_t i = 0; i < trailing_ones; i++) {
            cout << "1";
        }
        cout << "\n";
    }
    return 0;
}
