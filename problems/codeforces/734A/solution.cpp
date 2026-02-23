#include <iostream>
#include <iterator>

using namespace std;

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    string _,s;
    cin >> _ >> s;
    int anton = 0, danik = 0;
    for(auto c : s) {
        if(c == 'A') anton++;
        else danik++;
    }

    cout << (anton > danik ? "Anton" : anton < danik ? "Danik" : "Friendship");
    return 0;
}
