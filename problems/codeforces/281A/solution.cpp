#include <iostream>
#include <iterator>
#include <vector>

using namespace std;

int main() {
    vector<char> v{istreambuf_iterator<char>{cin}, {}};
    v[0] &= ~0x20;
    cout.write(v.data(), v.size());
    return 0;
}
