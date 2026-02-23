#include <iostream>
#include <string>

int main() {
    using namespace std;
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    string a, b;
    cin >> a >> b;
    cout << (a == b ? -1 : (int)max(a.length(), b.length())) << "\n";
    return 0;
}
