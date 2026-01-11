#include <bits/stdc++.h>

constexpr int BUFSIZE = 20000;

int main() {
    using namespace std;

    string input{istreambuf_iterator<char>{cin}, {}};
    vector<char> output_buf;
    output_buf.reserve(BUFSIZE);
    char* out_ptr = output_buf.data();

    for (auto const sv : input
                        | views::split('\n')
                        | views::drop(1)
                        | views::transform([](auto r) { return string_view(r); })
    ) {
        size_t len = sv.size();
        out_ptr = len > 10 
                ? format_to(out_ptr, "{}{}{}", sv.front(), len - 2, sv.back()) 
                : ranges::copy(sv, out_ptr).out;
        *out_ptr++ = '\n';
    }
    cout.write(output_buf.data(), out_ptr - output_buf.data());
    return 0;
}