#include <iostream>
#include <string>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t; cin >> t;
    while(t--) {
        string s; cin >> s;
        size_t count = 1;
        bool checked = false;
        for(size_t i = 1; i < s.length() ; i++) {
            if(s[i-1] == '1' && s[i] == '0') {
                count++;
            }
            if(s[i-1] == '0' && s[i] == '1') {
                if (checked) {
                    count++;
                } else {
                    checked = true;
                }
            }
        }
        cout << count << '\n';
    }
    return 0;
}
