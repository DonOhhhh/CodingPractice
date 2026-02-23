#include <iostream>
#include <ranges>
#include <string>
#include <unordered_map>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int n, m;
    cin >> n >> m;
    unordered_map<string, string> map;
    for(auto _ : views::iota(0, m)) {
        string f, s;
        cin >> f >> s;
        map[f] =  f.size() > s.size() ? s : f;
    }
    for(string word : views::istream<string>(cin)) {
        cout << map[word] << ' ';
    }
    return 0;
}
