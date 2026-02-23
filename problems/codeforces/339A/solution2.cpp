#include <iostream>
#include <string>
#include <vector>

using namespace std;

int main() {
    string s;
    cin >> s;

    auto count_one = 0;
    auto count_two = 0;
    auto count_three = 0;

    for (char c : s) {
        switch (c) {
            case '1': count_one++; break;
            case '2': count_two++; break;
            case '3': count_three++; break;
            default: {}
        }
    }

    vector<char> result;
    result.reserve(s.size() + 1);
    auto fill = [](vector<char>& v, int& count, char&& num) {
        for (int i = 0; i < count; i++) {
            v.push_back(num);
            v.push_back('+');
        }
    };
    fill(result, count_one, '1');
    fill(result, count_two, '2');
    fill(result, count_three, '3');
    
    cout.write(result.data(), result.size() - 1);
    return 0;
}
