#include <iostream>
#include <vector>
#include <charconv>
#include <algorithm>

// 1. 입력을 담을 고정 버퍼 (100KB 정도면 충분)
char in_buf[100005];
char out_buf[100005];

int main() {
    using namespace std;
    // C++ 최적화의 핵심 (반드시 넣어야 함)
    ios::sync_with_stdio(false);
    cin.tie(NULL);

    // 2. 전체 데이터를 한 번에 읽기
    cin.read(in_buf, sizeof(in_buf));
    streamsize n = cin.gcount();
    if (n <= 0) return 0;

    // 3. 첫 번째 줄(단어 개수) 건너뛰기
    char* p = in_buf;
    char* end = in_buf + n;
    while (p < end && *p != '\n') p++;
    if (p < end) p++; // '\n' 건너뛰기

    char* out_p = out_buf;

    // 4. 수동 바이트 스캔 (가장 빠름)
    while (p < end) {
        char* start = p;
        while (p < end && *p != '\n') p++;
        
        size_t len = p - start;
        if (len > 10) {
            *out_p++ = *start; // 첫 글자
            
            // 숫자를 문자로 변환 (to_chars 사용)
            auto [ptr, ec] = to_chars(out_p, out_p + 5, len - 2);
            out_p = ptr;
            
            *out_p++ = *(p - 1); // 마지막 글자
        } else if (len > 0) {
            // 단어 그대로 복사 (memcpy가 루프보다 빠름)
            for (char* t = start; t < p; ++t) *out_p++ = *t;
        }
        
        if (p < end) {
            *out_p++ = '\n';
            p++; // 다음 줄로
        }
    }

    // 5. 한 번에 출력
    cout.write(out_buf, out_p - out_buf);

    return 0;
}