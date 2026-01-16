#include <iostream>
#include <iterator>

using namespace std;

int main() {
    string s{istreambuf_iterator<char>{cin}, {}};

    

    cout.write(s.data(), s.size());
    return 0;
}
