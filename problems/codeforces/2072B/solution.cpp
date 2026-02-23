#include <iostream>
#include <string>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int t; cin >> t;
    while(t--) {
        int n; cin >> n;
        string s; cin >> s;

        // 1. 카운트 변수들을 처음부터 long long으로 선언하여 오버플로우 방지
        long long underscore_count = 0;
        for(char ch : s) {
            if(ch == '_') underscore_count++;
        }
        long long hyphen_count = n - underscore_count;

        // 2. 하이픈이 2개 미만이면 조합을 만들 수 없음
        if (hyphen_count < 2 || underscore_count == 0) {
            cout << 0 << '\n';
            continue;
        }

        // 3. 정수 연산만으로 반반 나누기
        long long left_h = hyphen_count / 2;
        long long right_h = hyphen_count - left_h;

        // 모든 변수가 long long이므로 중간 연산 결과도 long long (최대 900경까지 표현 가능)
        long long maximum = left_h * right_h * underscore_count;

        cout << maximum << '\n';
    }
    return 0;
}