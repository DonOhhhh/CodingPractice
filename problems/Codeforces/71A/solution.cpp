#include <iostream>
#include <iterator>
#include <ranges>
#include <vector>
#include <charconv>
#include <algorithm>

constexpr int BUFSIZE = 20000;

int main() {
    using namespace std;

    string input{istreambuf_iterator<char>{cin}, {}};
    vector<char> output_buf;
    output_buf.reserve(BUFSIZE);
    char* out_ptr = output_buf.data();

    for (auto const word : input
                        | views::split('\n')
                        | views::drop(1)
    ) {
        string_view sv{word.begin(), word.end()};
        size_t len = sv.size();
        
        if(len > 10) {
            *out_ptr++ = sv.front();

            auto [next_ptr, ec] = to_chars(out_ptr, out_ptr + 10, len - 2);
            out_ptr = next_ptr;
            
            *out_ptr++ = sv.back();
        } else {
            out_ptr = ranges::copy(sv, out_ptr).out;
        }
        *out_ptr++ = '\n';
    }
    cout.write(output_buf.data(), out_ptr - output_buf.data());
    return 0;
}