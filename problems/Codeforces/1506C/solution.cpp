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
        string a,b;
        cin >> a >> b;
        if(a.size() < b.size()) {
            swap(a,b);
        }
        string_view a_sv(a), b_sv(b);
        size_t lcs_count = 0;
        for(size_t i = 0; i < a.size(); i++) {
            for(size_t n = 1; i + n < a.size() + 1 ; n++) {
                if(b_sv.contains(a_sv.substr(i,n)) && lcs_count < n) {
                    lcs_count = n;
                }
            }
        }
        cout << (a.size() + b.size() - lcs_count * 2) << '\n';
    }
    return 0;
}
