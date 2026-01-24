#include <iostream>
#include <iterator>
#include <string>

int main() {
    using namespace std;
    string s{istreambuf_iterator<char>{cin}, {}};

    cout.write(s.data(), s.size());
    return 0;
}
