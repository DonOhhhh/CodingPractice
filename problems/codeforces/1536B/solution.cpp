#include <iostream>
#include <string>
#include <unordered_set>
#include <string_view>
#include <vector>
using namespace std;

void solve() {
    int n;
    string s;
    cin >> n >> s;

    // 1. 메모리 에러 방지: 입력을 받은 '후에' string_view를 생성합니다.
    string_view sv(s);
    unordered_set<string_view> seen;

    // 2. 부분 문자열 수집 (인덱스 범위 수정)
    for (int i = 0; i < n; i++) {
        seen.insert(sv.substr(i, 1));
        if (i + 1 < n) seen.insert(sv.substr(i, 2));
        if (i + 2 < n) seen.insert(sv.substr(i, 3));
    }

    string alphabet = "abcdefghijklmnopqrstuvwxyz";

    // 3. 탐색: substr로 자르지 않고 직접 문자를 합쳐서 모든 조합 생성!
    
    // 길이 1 검사
    for (char c1 : alphabet) {
        string target(1, c1);
        if (seen.count(target) == 0) { // 없다면 출력
            cout << target << "\n";
            return;
        }
    }

    // 길이 2 검사 (모든 a~z x a~z 조합)
    for (char c1 : alphabet) {
        for (char c2 : alphabet) {
            string target = {c1, c2}; // C++11 중괄호 초기화로 문자열 생성
            if (seen.count(target) == 0) {
                cout << target << "\n";
                return;
            }
        }
    }

    // 길이 3 검사
    for (char c1 : alphabet) {
        for (char c2 : alphabet) {
            for (char c3 : alphabet) {
                string target = {c1, c2, c3};
                if (seen.count(target) == 0) {
                    cout << target << "\n";
                    return;
                }
            }
        }
    }
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
