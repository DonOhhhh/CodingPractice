#include <iostream>
#include <string>
#include <unordered_map>
#include <algorithm> // ranges::max_element
#include <ranges>    // views::...

// C++20부터는 fmt나 print 사용 가능 (여기선 iostream 사용)
int main() {
    using namespace std;
    
    // 1. 입력 (Fast I/O)
    ios::sync_with_stdio(false); cin.tie(nullptr);
    int n; string s;
    cin >> n >> s;

    unordered_map<string, int> counts;

    // 2. 슬라이싱 & 카운팅 (Rust의 windows(2)와 유사)
    // C++23부터 views::slide 사용 가능.
    for (auto w : s | std::views::slide(2)) {
        // w는 ['A', 'B'] 같은 range 형태
        // string(w.begin(), w.end())로 변환해서 맵에 넣음
        string key(w.begin(), w.end());
        counts[key]++;
    }

    // 3. Rust 스타일의 체이닝 로직!
    // Rust: hashmap.iter().max_by_key(|entry| entry.1)
    auto result = ranges::max_element(counts, 
        {}, // 1. 비교 함수 (기본값: less, 생략 가능)
        &pair<const string, int>::second // 2. Projection (가장 중요!)
    );

    // result는 반복자(iterator)입니다.
    cout << result->first << "\n";

    return 0;
}