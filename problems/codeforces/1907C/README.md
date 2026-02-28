# [Codeforces 1907C : Removal of Unattractive Pairs](https://codeforces.com/problemset/problem/1907/C)

## 문제 설명

Vlad found a string $s$ consisting of $n$ lowercase Latin letters, and he wants to make it as short as possible.

To do this, he can remove $\mathbf{any}$ pair of adjacent characters from $s$ any number of times, provided they are $\mathbf{different}$. For example, if $s$=$\mathtt{racoon}$, then by removing one pair of characters he can obtain the strings $\mathtt{coon}$, $\mathtt{roon}$, $\mathtt{raon}$, and $\mathtt{raco}$, but he cannot obtain $\mathtt{racn}$ (because the removed letters were the same) or $\mathtt{rcon}$ (because the removed letters were not adjacent).

What is the minimum length Vlad can achieve by applying any number of deletions?

## 입력

The first line of the input contains a single integer $t$ ($1 \le t \le 10^4$) — the number of test cases. Descriptions of the test cases follow.

The first line of each test case contains a single integer $n$ ($1 \le n \le 2 \cdot 10^5$) — the length of the string $s$.

The second line of each test case contains the string $s$ consisting of $n$ lowercase Latin letters.

It is guaranteed that the sum of $n$ over all test cases does not exceed $2 \cdot 10^5$.

## 출력

For each test case, output a single number—the minimum length of the string $s$, after removing pairs of adjacent characters with different values.

## 노트

In the first test case of the example, you need to act as follows: "$\mathtt{aabc}$" $\rightarrow$ "$\mathtt{ac}$" $\rightarrow$ "".  that with a different order of deletions, the string will not become empty.

## 예제

### 1

#### 입력

```
10
4
aabc
5
abaca
10
avbvvcvvvd
7
abcdefg
5
dabbb
8
aacebeaa
7
bbbbacc
6
dacfcc
6
fdfcdc
9
dbdcfbbdc
```

#### 출력

```
0
1
2
1
1
0
1
0
0
1
```

## 티어(난이도)

1200

## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 1초 | 256MB |

## 알고리즘 분류

- constructive algorithms
- greedy
- math
- strings
