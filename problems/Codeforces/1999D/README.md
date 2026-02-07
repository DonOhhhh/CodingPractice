# [Codeforces 1999D : Slavic's Exam](https://codeforces.com/problemset/problem/1999/D)

## 문제 설명

Slavic has a very tough exam and needs your help in order to pass it. Here is the question he is struggling with:

There exists a string s, which consists of lowercase English letters and possibly zero or more "?".

Slavic is asked to change each "?" to a lowercase English letter such that string t becomes a subsequence (not necessarily continuous) of the string s.

Output any such string, or say that it is impossible in case no string that respects the conditions exists.

## 입력

The first line contains a single integer T ($1 \leq T \leq 10^4$) — the number of test cases.

The first line of each test case contains a single string s ($1 \leq |s| \leq 2 \cdot 10^5$, and s consists only of lowercase English letters and "?"-s)  – the original string you have.

The second line of each test case contains a single string t ($1 \leq |t| \leq |s|$, and t consists only of lowercase English letters)  – the string that should be a subsequence of string s.

The sum of |s| over all test cases doesn't exceed $2 \cdot 10^5$, where |x| denotes the length of the string x.

## 출력

For each test case, if no such string exists as described in the statement, output "NO" (without quotes).

Otherwise, output "YES" (without quotes). Then, output one line — the string that respects all conditions.

You can output "YES" and "NO" in any case (for example, strings "yEs", "yes", and "Yes" will be recognized as a positive response).

If multiple answers are possible, you can output any of them.

## 노트


## 예제

### 1

#### 입력

```
5
?????
xbx
ab??e
abcde
ayy?x
a
ab??e
dac
paiu
mom
```

#### 출력

```
YES
xabax
YES
abcde
YES
ayyyx
NO
NO
```

## 티어(난이도)

1000

## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 2초  | 256MB  |

## 알고리즘 분류

- greedy
- implementation
- strings