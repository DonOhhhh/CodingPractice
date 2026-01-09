#include <algorithm>
#include <iostream>
#include <iterator>
#include <ranges>

int main() {
    using namespace std;
    auto v = string{istreambuf_iterator<char>{cin}, {}}
             | views::split('\n') | views::drop(1)
             | views::join | views::transform([](auto c) {
                   return static_cast<int>(c - '0');
               });
    int sum = ranges::fold_left(v, 0, plus<int>());
    print(cout, "{}", sum);
    // cout.write(sum.data(), sum.size());
    return 0;
}