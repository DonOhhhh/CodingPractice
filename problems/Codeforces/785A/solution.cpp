#include <iostream>
#include <iterator>
#include <print>
#include <ranges>
#include <algorithm>

using namespace std;

int main() {
    // Fast I/O
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    // 전체 입력을 읽음
    string s{istreambuf_iterator<char>{cin}, {}};

    long long count = ranges::fold_left(
        s | views::split('\n') | views::drop(1), // 첫 줄(개수) 버림
        0LL,
        [](long long acc, auto sub) { // [핵심 1] auto로 변경 (subrange 타입)
            
            // [핵심 2] 빈 줄(마지막 줄바꿈 뒤 등)이 들어오면 무시
            if (sub.empty()) return acc;

            // [핵심 3] subrange의 첫 글자 확인 (*sub.begin())
            // 윈도우 줄바꿈(\r\n)이 섞여도 첫 글자는 동일하므로 안전함
            int faces = 0;
            switch (*sub.begin()) { 
                case 'T': faces = 4;  break;
                case 'C': faces = 6;  break;
                case 'O': faces = 8;  break;
                case 'D': faces = 12; break;
                case 'I': faces = 20; break;
            }
            return acc + faces;
        });

    print("{}", count);
    return 0;
}