#include <iostream>
#include <string>
#include <print>

int main() {
    using namespace std;
    string s;
    cin >> s;
    print("{}", s.contains("0000000") || s.contains("1111111") ? "YES" : "NO");
    return 0;
}
