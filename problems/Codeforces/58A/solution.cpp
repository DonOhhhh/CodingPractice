#include <iostream>
#include <iterator>
#include <vector>
#include <print>

int main() {
    using namespace std;
    vector<char> s{istreambuf_iterator<char>{cin}, {}};
    vector<char> hello = {'h','e','l','l','o'};
    int i = 0;
    for(char c: s) {
        if(i == 5) break;
        if(c == hello[i]) i++;
    }
    print("{}", i == 5 ? "YES" : "NO");
    return 0;
}
