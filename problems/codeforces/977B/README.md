# [Codeforces 977B : Two gram](https://codeforces.com/problemset/problem/977/B)

## 문제 설명

Two-gram is an ordered pair (i.e. string of length two) of capital Latin letters. For example, "AZ", "AA", "ZA" — three distinct two-grams.

You are given a string s consisting of n capital Latin letters. Your task is to find any two-gram contained in the given string as a substring (i.e. two consecutive characters of the string) maximal number of times. For example, for string s = "BBAABBBA" the answer is two-gram "BB", which contained in s three times. In other words, find any most frequent two-gram.

Note that occurrences of the two-gram can overlap with each other.

## 입력

The first line of the input contains integer number n ($2 \le n \le 100$) — the length of string s. The second line of the input contains the string s consisting of n capital Latin letters.

## 출력

Print the only line containing exactly two capital Latin letters — any two-gram contained in the given string s as a substring (i.e. two consecutive characters of the string) maximal number of times.

## 노트

In the first example "BA" is also valid answer.

In the second example the only two-gram "ZZ" can be printed because it contained in the string "ZZZAA" two times.

## 예제

### 1

#### 입력

```
7
ABABCABA
```

#### 출력

```
AB
```

### 2

#### 입력

```
5
ZZZAA
```

#### 출력

```
ZZ
```

## 티어(난이도)

900

## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 1초  | 256MB  |

## 알고리즘 분류

- implementation
- strings