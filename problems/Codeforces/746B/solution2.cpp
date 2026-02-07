#include <iostream>
#include <string>
#include <deque>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n;
    string s;
    if (!(cin >> n >> s)) return 0;

    deque<char> res;

    for (int i = 0; i < n; ++i) {
        // 남은 글자의 개수 = 전체 길이(n) - 현재까지 처리한 글자 수(i)
        // 이 값이 홀수냐 짝수냐에 따라 방향 결정
        if ((n - i) % 2 == 1) {
            res.push_back(s[i]);
        } else {
            res.push_front(s[i]);
        }
    }

    // deque의 내용을 순서대로 출력
    for (char c : res) {
        cout << c;
    }
    cout << '\n';

    return 0;
}