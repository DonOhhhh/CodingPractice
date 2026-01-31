#include <iostream>
#include <string>
#include <algorithm> // ranges::count
#include <print>     // C++23 print/println
#include <ranges>    // C++23 views::enumerate

using namespace std;

void solve() {
    string s;
    cin >> s;

    // [핵심 수정] auto 대신 타입을 명시적으로 통일! (long long 또는 int)
    // ranges::count의 반환값을 강제로 형변환하여 저장
    long long zeros = ranges::count(s, '0');
    long long ones = (long long)s.size() - zeros;

    // views::enumerate (C++23)
    for (auto [i, c] : s | views::enumerate) {
        
        // 이제 zeros와 ones의 타입이 같으므로 삼항 연산자가 '참조'를 반환함
        // c가 '0'이면 -> ones를 줄여야 함
        // c가 '1'이면 -> zeros를 줄여야 함
        long long& available = (c == '0' ? ones : zeros);

        if (available > 0) {
            --available;
        } else {
            // 자원이 없으면 현재 남은 길이(전체 - 현재인덱스) 출력 후 종료
            println("{}", s.size() - i);
            return;
        }
    }

    println("0");
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int t;
    if (cin >> t) {
        while (t--) {
            solve();
        }
    }
    return 0;
}