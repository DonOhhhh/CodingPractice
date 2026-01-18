#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

int main() {
    string s;
    cin >> s;

    string nums;
    // Extract only digits
    for (char c : s) {
        if (c != '+') nums.push_back(c);
    }

    // Sort the digits
    ranges::sort(nums);

    // Print with '+'
    vector<char> v;
    for (size_t i = 0; i < nums.size(); ++i) {
        v.push_back(nums[i]);
        v.push_back('+');
    }

    cout.write(v.data(), v.size() - 1);
    return 0;
}