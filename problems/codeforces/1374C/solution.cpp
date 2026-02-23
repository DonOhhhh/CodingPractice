#include <cstdint>
#include <iostream>
#include <string>
#include <ranges>
#include <print>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    for(auto s : views::istream<string>(cin) | views::drop(2) | views::stride(2)) {
        int8_t count = 0, moves = 0;
        for(char c : s) {
            if( c == '(') {
                count++;
            } else {
                count--;
                if(count < 0) {
                    moves++;
                    count = 0;
                }
            }
        }
        println("{}", moves);
    }
    return 0;
}
