#include <iostream>
#include <string>

using namespace std;

int main() {
    // 1. C++ I/O 속도 향상 (필수)
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    string s1, s2;
    cin >> s1 >> s2; 

    for (size_t i = 0; i < s1.size(); ++i) {
        char c1 = s1[i] | 0x20; 
        char c2 = s2[i] | 0x20;

        if (c1 < c2) {
            cout << "-1";
            return 0;
        }
        if (c1 > c2) {
            cout << "1";
            return 0;
        }
    }

    cout << "0";
    return 0;
}