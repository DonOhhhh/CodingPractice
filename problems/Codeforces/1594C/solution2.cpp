#include <iostream>
#include <string>
#include <vector>
#include <ranges>
#include <algorithm>
#include <optional>
#include <print> // C++23: println

using namespace std;

// Rust의 'find' 처럼 결과를 Option으로 반환하는 헬퍼 람다
constexpr auto find_valid_x = [](int n, char c, string_view s) -> optional<int> {
    // Rust: (1..=n).rev()
    auto candidates = views::iota(1, n + 1) | views::reverse;

    auto it = ranges::find_if(candidates, [&](int x) {
        // Rust: (x..=n).step_by(x).all(|i| s[i-1] == c)
        // C++23: views::stride(x)를 사용하여 배수 인덱스 생성
        auto multiples = views::iota(x, n + 1) | views::stride(x);
        return ranges::all_of(multiples, [&](int i) { return s[i - 1] == c; });
    });

    return (it != candidates.end()) ? optional{*it} : nullopt;
};

void solve() {
    int n; char c; string s;
    cin >> n >> c >> s;

    // 1. Rust: s.chars().all(...) -> bool
    // 소유권 개념 적용: s를 변경하지 않고 읽기 전용 뷰(const ref)로 처리
    const bool all_match = ranges::all_of(s, [c](char ch) { return ch == c; });
    
    if (all_match) {
        println("0");
        return;
    }

    // 2. Rust: find(...).map(...).unwrap_or(...)
    // Monadic Operations (transform, or_else) 활용
    find_valid_x(n, c, s)
        .transform([](int x) {
            println("1\n{}", x);
            return 0; // Dummy return for transform type matching
        })
        .or_else([&]() -> optional<int> {
            println("2\n{} {}", n - 1, n);
            return nullopt;
        });
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    
    int t; cin >> t;
    while(t--) solve();
}