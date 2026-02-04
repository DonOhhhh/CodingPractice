#include <iostream>
#include <string>
#include <numeric>

using namespace std;

long long lcm(int a, int b) {
    return (1LL * a * b) / gcd(a,b);
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int q;
    cin >> q;
    string s,t;
    while(q--) {
        cin >> s >> t;
        int n = s.length(), m = t.length();
        long long L = lcm(n, m);
        string res_s,res_t;
        for(int _ = 0 ; _ < L / n; _++) res_s += s;
        for(int _ = 0 ; _ < L / m; _++) res_t += t;
        cout << (res_s == res_t ? res_s : "-1") << "\n";
    }
    return 0;
}
