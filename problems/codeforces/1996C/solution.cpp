#include <algorithm>
#include <iostream>
#include <ranges>
#include <string>
#include <vector>

using namespace std;

void print_matrix(vector<vector<int>>& matrix) {
    auto matrix_str =
        matrix | views::transform([](auto&& v) {
            return v 
                | views::transform([](auto&& i) {return to_string(i);})
                | views::join_with(' ');
        })
        | views::join_with('\n') 
        | ranges::to<string>();
    cout << matrix_str << "\n\n";
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    int t;
    cin >> t;
    while (t--) {
        int n, q;
        cin >> n >> q;
        string a, b;
        cin >> a >> b;
        vector<vector<int>> pref_a(26,
                                   vector<int>(n + 1, 0));
        vector<vector<int>> pref_b(26,
                                   vector<int>(n + 1, 0));
        for (const auto& i : views::iota(0, n)) {
            for (const auto& j : views::iota(0, 26)) {
                pref_a[j][i + 1] = pref_a[j][i];
                pref_b[j][i + 1] = pref_b[j][i];
            }
            pref_a[a[i] - 'a'][i + 1]++;
            pref_b[b[i] - 'a'][i + 1]++;
            // print_matrix(pref_a);
            // print_matrix(pref_b);
        }

        while(q--) {
            int l, r; cin >> l >> r;
            int diff_sum = 0;
            for(const auto& i : views::iota(0,26)) {
                int count_a = pref_a[i][r] - pref_a[i][l-1];
                int count_b = pref_b[i][r] - pref_b[i][l-1];

                if(count_a > count_b) {
                    diff_sum += (count_a - count_b);
                }
            }
            cout << diff_sum << "\n";
        }
    }
    return 0;
}
