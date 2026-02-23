#include <iostream>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    string n,s;
    cin >> n >> s;
    uint32_t count = 0;
    for(char& c : s) {
        count |= (1 << ((c | 0x20) - 'a'));
    }
    cout << (count == (1 << 26) - 1 ? "YES" : "NO");
    return 0;
}
