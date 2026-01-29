#include <iostream>
#include <string>
#include <unordered_map>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    string s;
    int n;
    cin >> n >> s;
    unordered_map<string, int> um;
    for(int i = 0; i < n-1; i++) {
        um[s.substr(i,2)]++;
    }
    auto best_it = max_element(um.begin(), um.end(), 
        [](const auto& a, const auto& b) {
            return a.second < b.second; // 값(value)을 기준으로 비교
        }
    );
    cout << best_it->first;
    return 0;
}
