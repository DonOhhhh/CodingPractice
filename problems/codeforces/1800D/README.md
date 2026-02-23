# [Codeforces 1800D : Remove Two Letters](https://codeforces.com/problemset/problem/1800/D)

## 문제 설명

Dmitry has a string s, consisting of lowercase Latin letters.

Dmitry decided to remove two consecutive characters from the string s and you are wondering how many different strings can be obtained after such an operation.

For example, Dmitry has a string "aaabcc". You can get the following different strings: "abcc"(by deleting the first two or second and third characters), "aacc"(by deleting the third and fourth characters),"aaac"(by deleting the fourth and the fifth character) and "aaab" (by deleting the last two).

## 입력

The first line of input data contains a single integer t ($1 \le t \le 10^4$) — number of test cases.

The descriptions of the test cases follow.

The first line of the description of each test case contains an integer n ($3 \le n \le 2 \cdot 10^5$).

The second line of the description of each test case contains a string s of length n consisting of lowercase Latin letters.

It is guaranteed that the sum of n for all test cases does not exceed $2 \cdot 10^5$.

## 출력

For each test case print one integer — the number of distinct strings that can be obtained by removing two consecutive letters.

## 노트

The first example is explained in the statement.

In the third example, the following strings are obtained: "cdef", "adef", "abef", "abcf", "abcd".

In the seventh example, any deletion will result in the string "aba".

## 예제

### 1

#### 입력

```
7
6
aaabcc
10
aaaaaaaaaa
6
abcdef
7
abacaba
6
cccfff
4
abba
5
ababa
```

#### 출력

```
4
1
5
3
3
3
1
```

## 티어(난이도)

1200

## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 2초  | 256MB  |

## 알고리즘 분류

- data structures
- greedy
- hashing
- strings