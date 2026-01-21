#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

int main() {
    string s;
    cin >> s;
    int l_count = ranges::count_if(s, [](auto c){return c >= 'a' && c <= 'z';});
    int u_count = s.size() - l_count;
    vector<char> v;
    v.reserve(1 << 7);
    for(char& c : s) {
        if(l_count >= u_count) {
            v.push_back(c | 0x20);
        } else {
            v.push_back(c & ~0x20);
        }
    }
    cout.write(v.data(), v.size());
    return 0;
}
