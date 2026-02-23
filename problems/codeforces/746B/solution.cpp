#include <iostream>
#include <string>
#include <vector>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int _;
    string word;
    cin >> _ >> word;
    vector<char> res;
    for(int i = word.length() - 1, j=0 ; i >= 0 ; i--, j++) {
        auto pos = j%2==0 ? res.size() - j/2 : j/2;
        res.insert(res.begin() + pos, word[i]);
    }
    cout.write(res.data(), res.size());
    return 0;
}
