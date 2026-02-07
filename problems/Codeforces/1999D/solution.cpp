#include <iostream>
#include <string>
#include <vector>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t;
    cin >> t;
    while(t--) {
        string s,t;
        cin >> s >> t;
        size_t cur = 0;
        for(char& c : s) {
            if(cur < t.length()) {
                if( c == t[cur] || c == '?') {
                    c = t[cur++];
                }
            } else {
                if(c == '?') {
                    c = 'a';
                }
            }
        }
        if(cur == t.length()) {
            cout << "YES" << '\n';
            cout << s << '\n';
        } else {
            cout << "NO" << '\n';
        }
    }
    return 0;
}
