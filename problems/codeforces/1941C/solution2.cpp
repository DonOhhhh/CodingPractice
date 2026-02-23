#include <iostream>
#include <string>
#include <vector>

using namespace std;

void solve() {
    int n;
    cin >> n;
    string s;
    cin >> s;   
    string_view sv(s);
    // dp[i]: i번째 문자(길이 i)까지 처리했을 때의 최소 삭제 횟수
    vector<int> dp(n + 1, 0);

    for (int i = 1; i <= n; i++) {
        // 1. 기본적으로는 이전 상태를 유지 (삭제 안 함)
        dp[i] = dp[i - 1];

        // 2. 길이가 3 이상일 때 패턴 확인
        if (i >= 3) {
            // 현재 위치 i에서 끝나는 3글자가 "map" 또는 "pie"인지 확인
            // string 인덱스는 0부터 시작하므로 sv.substr(i-3, 3) 사용
            string_view sub = sv.substr(i - 3, 3);
            
            if (sub == "map" || sub == "pie") {
                // 패턴이 발견되면, 패턴이 시작되기 전 상태(i-3)에 +1(현재 문자 삭제)
                // 단, dp[i]는 최소값을 유지해야 하므로 기존 값과 비교할 수도 있겠지만
                // 패턴이 발견된 순간 '무조건' 깨야 하므로 덮어씌웁니다.
                // (이전 값 dp[i-1]은 패턴을 허용한 상태라 유효하지 않음)
                dp[i] = dp[i - 3] + 1;
            }
        }
    }

    cout << dp[n] << "\n";
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int t;
    cin >> t;
    while (t--) {
        solve();
    }
    return 0;
}