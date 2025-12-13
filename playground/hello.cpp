#include <bits/stdc++.h>
using namespace std;

int func2(const vector<int> &v1, int n) {
  vector<int> freq(100, 0);
  for (auto i : v1) {
    if (freq[100 - i])
      return 1;
    freq[i]++;
  }
  return 0;
}

int main(void) {
  ios::sync_with_stdio(0);
  cin.tie(0);
  cout << func2({1, 23, 53, 77, 60}, 5) << '\n';
  cout << func2({1, 52, 48}, 3) << '\n';
  cout << func2({50, 42}, 2) << '\n';
  cout << func2({4, 13, 63, 87}, 4) << '\n';
}