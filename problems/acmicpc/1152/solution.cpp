#include <iostream>
#include <print>
#include <ranges>
#include <vector>

int main() {
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);

    using namespace std;

    auto words = views::istream<string>(cin);
    print("{}", ranges::distance(words));
    return 0;
}