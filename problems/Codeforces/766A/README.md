# [Codeforces 766A : Mahmoud and Longest Uncommon Subsequence](https://codeforces.com/problemset/problem/766/A)

## 문제 설명

While Mahmoud and Ehab were practicing for IOI, they found a problem which name was Longest common subsequence. They solved it, and then Ehab challenged Mahmoud with another problem.

Given two strings a and b, find the length of their longest uncommon subsequence, which is the longest string that is a subsequence of one of them and not a subsequence of the other.

A subsequence of some string is a sequence of characters that appears in the same order in the string, The appearances don't have to be consecutive, for example, strings "ac", "bc", "abc" and "a" are subsequences of string "abc" while strings "abbc" and "acb" are not. The empty string is a subsequence of any string. Any string is a subsequence of itself.

## 입력

The first line contains string a, and the second line — string b. Both of these strings are non-empty and consist of lowercase letters of English alphabet. The length of each string is not bigger than $10^5$ characters.

## 출력

If there's no uncommon subsequence, print "-1". Otherwise print the length of the longest uncommon subsequence of a and b.

## 노트

In the first example: you can choose "defgh" from string b as it is the longest subsequence of string b that doesn't appear as a subsequence of string a.

## 예제

### 1

#### 입력

```
abcd
defgh
```

#### 출력

```
5
```

### 2

#### 입력

```
a
a
```

#### 출력

```
-1
```

## 티어(난이도)

1000

## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 2초  | 256MB  |

## 알고리즘 분류

- constructive algorithms
- strings