#include <iostream>
#include <string>
#include <vector>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int n; cin >> n;
    string s; cin >> s;
    bool res = false;
    size_t l = 0, r = 0;
    for(size_t i = 0 ; i < n-1; i++) {
        if(s[i] > s[i+1]) {
            res = true;
            l = i+1;
            r = i+2;
            break;
        }
    }
    if(res) {
        cout << "YES" << "\n";
        cout << l << " " << r << "\n";
    } else {
        cout << "NO" << "\n";
    }
    return 0;
}
