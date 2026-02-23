#include <iostream>
#include <string>

using namespace std;

void solve() {
    string s;
    cin >> s;
    int n = s.length();

    // 1. 뒤에서부터 탐색하여 합이 10 이상인 경우를 찾음 (전략 1)
    for (int i = n - 2; i >= 0; i--) {
        int sum = (s[i] - '0') + (s[i + 1] - '0');
        if (sum >= 10) {
            // 합친 결과를 출력하고 함수 종료
            // 앞부분(0 ~ i-1) + 합(sum) + 뒷부분(i+2 ~ 끝)
            cout << s.substr(0, i) << sum << s.substr(i + 2) << "\n";
            return;
        }
    }

    // 2. 10 이상인 경우가 하나도 없다면 무조건 맨 앞 두 숫자를 합침 (전략 2)
    int sum = (s[0] - '0') + (s[1] - '0');
    cout << sum << s.substr(2) << "\n";
}

int main() {
    // 입출력 속도 향상
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    
    int t;
    cin >> t;
    while (t--) {
        solve();
    }
    return 0;
}