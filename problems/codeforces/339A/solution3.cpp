#include <algorithm>
#include <format>
#include <iostream>
#include <ranges>
#include <string>
#include <vector>

using namespace std;

int main() {
    string s;
    cin >> s;

    vector<char> nums =
        s | views::filter([](auto e) { return e != '+'; })
          | ranges::to<vector<char>>();
    
    ranges::stable_sort(nums);
    
    string result = ranges::fold_left(
        nums, "", [](string acc, char elem) {
            return format("{}{}+", acc, elem);
        });
    cout.write(result.data(), result.size() - 1);
    return 0;
}
