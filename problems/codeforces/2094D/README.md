# [Codeforces 2094D : Tung Tung Sahur](https://codeforces.com/problemset/problem/2094/D)

## 문제 설명

You have two drums in front of you: a left drum and a right drum. A hit on the left can be recorded as "$\texttt{L}$", and a hit on the right as "$\texttt{R}$".

The strange forces that rule this world are fickle: sometimes, a blow sounds once, and sometimes it sounds twice. Therefore, a hit on the left drum could have sounded as either "$\texttt{L}$" or "$\texttt{LL}$", and a hit on the right drum could have sounded as either "$\texttt{R}$" or "$\texttt{RR}$".

The sequence of hits made is recorded in the string  $p $, and the sounds heard are in the string  $s $. Given  $p $ and  $s $, determine whether it is true that the string  $s $ could have been the result of the hits from the string  $p $.

For example, if  $p= $"$\texttt{LR}$", then the result of the hits could be any of the strings "$\texttt{LR}$", "$\texttt{LRR}$", "$\texttt{LLR}$", and "$\texttt{LLRR}$", but the strings "$\texttt{LLLR}$" or "$\texttt{LRL}$" cannot.

## 입력

The first line contains an integer  $t $ ( $1 \leq t \leq 10^4 $) – the number of independent test cases.

The first line of each test case contains the string  $p $ ( $1 \le |p| \le 2 \cdot 10^5 $) consisting only of the characters "$\texttt{R}$" and "$\texttt{L}$", where  $|p| $ denotes the length of the string  $p $.

The second line of each test case contains the string  $s $ ( $1 \le |p| \le |s| \le 2 \cdot 10^5 $) consisting only of the characters "$\texttt{R}$" and "$\texttt{L}$".

It is guaranteed that the sum of  $|s| $ does not exceed  $2\cdot 10^5 $ across all test cases.

## 출력

For each set of input data, output "$\texttt{YES}$" if  $s $ can be the heard sound, and "$\texttt{NO}$" otherwise. You may output in any case.

## 노트



## 예제

### 1

#### 입력

```
5
R
RR
LRLR
LRLR
LR
LLLR
LLLLLRL
LLLLRRLL
LLRLRLRRL
LLLRLRRLLRRRL
```

#### 출력

```
YES
YES
NO
NO
YES
```

## 티어(난이도)

1100

## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 2초 | 256MB |

## 알고리즘 분류

- greedy
- strings
- two pointers
