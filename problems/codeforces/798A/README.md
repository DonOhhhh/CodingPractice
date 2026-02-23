# [Codeforces 798A : Mike and palindrome](https://codeforces.com/problemset/problem/798/A)

## 문제 설명

Mike has a string s consisting of only lowercase English letters. He wants to **change exactly one** character from the string so that the resulting one is a palindrome.

A palindrome is a string that reads the same backward as forward, for example strings "$\texttt{z}$", "$\texttt{aaa}$", "$\texttt{aba}$", "$\texttt{abccba}$" are palindromes, but strings "$\texttt{codeforces}$", "$\texttt{reality}$", "$\texttt{ab}$" are not.

## 입력

The first and single line contains string s (1 ≤ |s| ≤ 15).

## 출력

Print "$\texttt{YES}$" (without quotes) if Mike can change **exactly** one character so that the resulting string is palindrome or "$\texttt{NO}$" (without quotes) otherwise.

## 노트



## 예제

### 1

#### 입력

```
abccaa
```

#### 출력

```
YES
```

### 2

#### 입력

```
abbcca
```

#### 출력

```
NO
```

### 3

#### 입력

```
abcda
```

#### 출력

```
YES
```

## 티어(난이도)

1000

## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 2초 | 256MB |

## 알고리즘 분류

- brute force
- constructive algorithms
- strings
