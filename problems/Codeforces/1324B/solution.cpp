#include <iostream>
#include <string>
#include <vector>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t;
    cin >> t;
    while (t--) {
        size_t a_len;
        cin >> a_len;
        vector<int> a(a_len);
        for (auto& e : a) {
            cin >> e;
        }
        vector<int> pos(5001, 1e9);
        bool res(false);
        for (size_t i = 0; i < a_len; i++) {
            auto num = a[i];
            if (pos[num] == 1e9) {
                pos[num] = i;
                continue;
            }
            if (i - pos[num] >= 2) {
                res = true;
                break;
            }
        }
        cout << (res ? "YES" : "NO") << '\n';
    }
    return 0;
}
