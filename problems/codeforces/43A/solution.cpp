#include <iostream>
#include <iterator>
#include <string>
#include <ranges>

int main() {
    using namespace std;
    string s{istreambuf_iterator{cin}, {}};
    auto words = s 
        | views::split('\n')    // 쪼개면 subrange들이 나옴
        | views::drop(1)        // N 건너뜀
        | views::transform([](auto&& rng) { 
            // 핵심: subrange -> string_view로 "보는 관점"만 바꿈 (비용 0)
            return string_view(rng.begin(), rng.end()); 
        });
    
    string leader = "";
    int score = 0;

    for(string_view word: words) {
        if(word.empty()) continue;
        if(score == 0) {
            leader = word;
            score = 1;
        } else if(word == leader) {
            score++;
        } else {
            score--;
        }
    }
    cout << leader;
    return 0;
}
