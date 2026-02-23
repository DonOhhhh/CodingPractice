#include <iostream>
#include <string>
#include <string_view>
#include <ranges>
#include <print> // C++23

using namespace std;
using namespace std::literals; // "WUB"sv를 쓰기 위해 필요

int main() {
    // Fast I/O
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    string s;
    cin >> s;

    // C++23 파이프라인
    auto result = s 
        | views::split("WUB"sv)  // 1. "WUB"을 기준으로 자름 (Rust: split)
        | views::filter([](auto sub) { return !sub.empty(); }) // 2. 빈 문자열 제거 (Rust: filter)
        | views::join_with(' ')  // 3. 공백으로 합침 (Rust: join, C++23 신기능!)
        | ranges::to<string>();  // 4. 최종 문자열로 변환 (Rust: collect)

    println("{}", result);

    return 0;
}