#include <iostream>
#include <string>
#include <vector>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int t; 
    cin >> t;
    while(t--) {
        int n;
        char c;
        cin >> n >> c;
        string s; 
        cin >> s;

        // 1단계: 모두 c인지 확인
        bool all_c = true;
        for(char ch : s) {
            if(ch != c) {
                all_c = false;
                break;
            }
        }

        if(all_c) {
            cout << "0\n";
            continue; // return 0이 아니라 continue!
        }

        // 2단계: 가능한 x 찾기
        int found_x = -1; // char 대신 int 사용
        for(int x = 1; x <= n ; x++) {
            bool possible = true;
            for(int j = x ; j <= n ; j += x) {
                if(s[j-1] != c) {
                    possible = false;
                    break;
                }
            }

            if(possible) { // !possible이 아니라 possible일 때!
                found_x = x;
                break;
            }
        }

        if(found_x != -1) {
            cout << "1\n" << found_x << "\n";
        } else {
            cout << "2\n" << n-1 << " " << n << "\n";
        }
    }
    return 0;
}