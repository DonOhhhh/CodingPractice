#include <iostream>
#include <string>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t;
    cin >> t;
    string s;
    s.reserve(200005);
    int cnt[26];
    while(t--) {
        int n;
        cin >> n;
        fill(begin(cnt), end(cnt), 0);
        s.clear();
        while(n--) {
            int a;
            cin >> a;
            for(int i = 0 ; i < 26; i++) {
                if(cnt[i] == a) {
                    s.push_back((char)('a'+i));
                    cnt[i]++;
                    break;
                }
            }
        }
        cout << s << '\n';
    }
    return 0;
}
