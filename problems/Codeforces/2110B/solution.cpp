#include <iostream>
#include <string>
#include <vector>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t; cin >> t;
    while(t--) {
        string s; cin >> s;
        int count = 0;
        string res = "NO";
        for(size_t i = 0; i < s.size(); i++) {
            if(s[i] == '(') {
                count++;
            } else {
                count--;
            }
            if(i != s.size()-1 && count == 0) {
                res = "YES";
                break;
            }
        }
        cout << res << "\n";
    }
    return 0;
}
