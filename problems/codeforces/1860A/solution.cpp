#include <iostream>
#include <string>
#include <vector>
using namespace std;

void solve() {
    string s; cin >> s;
    if(s == "()") {
        cout << "NO" << "\n";
        return;
    }
    auto n = s.size();
    string s1 = "";
    for(auto i = 0 ; i < n ; i++) {
        s1.push_back('(');
    }
    for(auto i = 0 ; i < n ; i++) {
        s1.push_back(')');
    }
    string s2 = "";
    for(auto i = 0 ; i < n ; i++) {
        s2.push_back('(');
        s2.push_back(')');
    }
    if(s1.find(s) == string::npos) {
        cout << "YES" << "\n";
        cout << s1 << "\n";
    } else if(s2.find(s) == string::npos) {
        cout << "YES" << "\n";
        cout << s2 << "\n";
    } else {
        cout << "NO" << "\n";
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t; cin >> t;
    while(t--) {
        solve();
    }
    return 0;
}
