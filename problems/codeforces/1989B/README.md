# [Codeforces 1989B : Substring and Subsequence](https://codeforces.com/problemset/problem/1989/B)

## 문제 설명

You are given two strings $a$ and $b$, both consisting of lowercase Latin letters. 

A $\mathit{subsequence}$ of a string is a string which can be obtained by removing several (possibly zero) characters from the original string. A $\mathit{substring}$ of a string is a contiguous subsequence of that string.

For example, consider the string $\mathtt{abac}$:

 - $\mathtt{a}$, $\mathtt{b}$, $\mathtt{c}$, $\mathtt{ab}$, $\mathtt{aa}$, $\mathtt{ac}$, $\mathtt{ba}$, $\mathtt{bc}$, $\mathtt{aba}$, $\mathtt{abc}$, $\mathtt{aac}$, $\mathtt{bac}$ and $\mathtt{abac}$ are its subsequences;
- $\mathtt{a}$, $\mathtt{b}$, $\mathtt{c}$, $\mathtt{ab}$, $\mathtt{ba}$, $\mathtt{ac}$, $\mathtt{aba}$, $\mathtt{bac}$ and $\mathtt{abac}$ are its substrings.

Your task is to calculate the minimum possible length of the string that contains $a$ as a substring and $b$ as a subsequence.

## 입력

The first line contains a single integer $t$ ($1 \le t \le 10^3$) — the number of test cases.

The first line of each test case contains a string $a$ ($1 \le |a| \le 100$), consisting of lowercase Latin letters.

The second line of each test case contains a string $b$ ($1 \le |b| \le 100$), consisting of lowercase Latin letters.

## 출력

For each test case, print a single integer — the minimum possible length of the string that contains $a$ as a substring and $b$ as a subsequence.

## 노트

In the examples below, the characters that correspond to the subsequence equal to $b$ are bolded.

In the first example, one of the possible answers is $\mathtt{\mathbf{c}a\mathbf{b}a}$.

In the second example, one of the possible answers is $\mathtt{er\mathbf{cf}}$.

In the third example, one of the possible answers is $\mathtt{\mathbf{mmm}}$.

In the fourth example, one of the possible answers is $\mathtt{con\mathbf{test}}$.

In the fifth example, one of the possible answers is $\mathtt{\mathbf{abc}d\mathbf{efg}}$.

## 예제

### 1

#### 입력

```
5
aba
cb
er
cf
mmm
mmm
contest
test
cde
abcefg
```

#### 출력

```
4
4
3
7
7
```

## 티어(난이도)

1200

## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 2초 | 256MB |

## 알고리즘 분류

- brute force
- greedy
- strings
