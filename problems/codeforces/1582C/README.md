# [Codeforces 1582C : Grandma Capa Knits a Scarf](https://codeforces.com/problemset/problem/1582/C)

## 문제 설명

Grandma Capa has decided to knit a scarf and asked Grandpa Sher to make a pattern for it, a pattern is a string consisting of lowercase English letters. Grandpa Sher wrote a string $s$ of length $n$.

Grandma Capa wants to knit a beautiful scarf, and in her opinion, a beautiful scarf can only be knit from a string that is a palindrome. She wants to change the pattern written by Grandpa Sher, but to avoid offending him, she will choose one lowercase English letter and erase some (at her choice, possibly none or all) occurrences of that letter in string $s$.

She also wants to minimize the number of erased symbols from the pattern. Please help her and find the minimum number of symbols she can erase to make string $s$ a palindrome, or tell her that it's impossible. Notice that she can only erase symbols equal to the $\mathbf{one}$ letter she chose.

A string is a palindrome if it is the same from the left to the right and from the right to the left. For example, the strings $\mathtt{'kek'}$, $\mathtt{'abacaba'}$, $\mathtt{'r'}$ and $\mathtt{'papicipap'}$ are palindromes, while the strings $\mathtt{'abb'}$ and $\mathtt{'iq'}$ are not.

## 입력

The first line contains a single integer $t$ ($1 \le t \le 100$) — the number of test cases. The next $2 \cdot t$ lines contain the description of test cases. The description of each test case consists of two lines.

The first line of each test case contains a single integer $n$ ($1 \le n \le 10^5$) — the length of the string.

The second line of each test case contains the string $s$ consisting of $n$ lowercase English letters.

It is guaranteed that the sum of $n$ over all test cases does not exceed $2 \cdot 10^5$.

## 출력

For each test case print the minimum number of erased symbols required to make the string a palindrome, if it is possible, and $-1$, if it is impossible.

## 노트

In the first test case, you can choose a letter $\mathtt{'a'}$ and erase its first and last occurrences, you will get a string $\mathtt{'bcaacb'}$, which is a palindrome. You can also choose a letter $\mathtt{'b'}$ and erase all its occurrences, you will get a string $\mathtt{'acaaca'}$, which is a palindrome as well.

In the second test case, it can be shown that it is impossible to choose a letter and erase some of its occurrences to get a palindrome.

In the third test case, you don't have to erase any symbols because the string is already a palindrome.

## 예제

### 1

#### 입력

```
5
8
abcaacab
6
xyzxyz
4
abba
8
rprarlap
10
khyyhhyhky
```

#### 출력

```
2
-1
0
3
2
```

## 티어(난이도)

1200

## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 1초 | 256MB |

## 알고리즘 분류

- brute force
- data structures
- greedy
- strings
- two pointers
