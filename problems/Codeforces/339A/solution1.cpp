#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

int main() {
    std::string s;
    std::cin >> s;

    std::string nums;
    // Extract only digits
    for (char c : s) {
        if (c != '+') nums.push_back(c);
    }

    // Sort the digits
    std::ranges::sort(nums);

    // Print with '+'
    std::vector<char> v;
    for (size_t i = 0; i < nums.size(); ++i) {
        v.push_back(nums[i]);
        v.push_back('+');
    }

    std::cout.write(v.data(), v.size() - 1);
    return 0;
}