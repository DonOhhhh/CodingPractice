#include <iostream>
#include <string>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t;
    cin >> t;
    while(t--) {
        string s;
        cin >> s;
        size_t n = s.size();
        if(s[0] != s[n-1]) {
            s[0] = s[n-1];
        }
        cout << s << '\n';
    }
    return 0;
}
