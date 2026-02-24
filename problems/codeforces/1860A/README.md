# [Codeforces 1860A : Not a Substring](https://codeforces.com/problemset/problem/1860/A)

## 문제 설명

A bracket sequence is a string consisting of characters '$\texttt{(}$' and/or '$\texttt{)}$'. A regular bracket sequence is a bracket sequence that can be transformed into a correct arithmetic expression by inserting characters '$\texttt{1}$' and '$\texttt{+}$' between the original characters of the sequence. For example:

- bracket sequences "$\texttt{()()}$" and "$\texttt{(())}$" are regular (they can be transformed into "$\texttt{(1)+(1)}$" and "$\texttt{((1+1)+1)}$", respectively);
- bracket sequences "$\texttt{)(}$", "$\texttt{(}$" and "$\texttt{)}$" are not regular.

You are given a bracket sequence $s$; let's define its length as $n$. Your task is to find a regular bracket sequence $t$ of length $2n$ such that $s$ **does not** occur in $t$ as a **contiguous substring**, or report that there is no such sequence.

## 입력

The first line contains a single integer $t$ ($1 \le t \le 1000$) — the number of test cases.

The only line of each test case contains a string $s$ ($2 \le |s| \le 50$), consisting of characters "$\texttt{(}$" and/or "$\texttt{)}$".

## 출력

For each test case, print the answer to it. If there is no required regular bracket sequence, print $\texttt{NO}$ in a separate line. Otherwise, print $\texttt{YES}$ in the first line, and the required regular bracket sequence $t$ itself in the second line. If there are multiple answers — you may print any of them.

## 노트



## 예제

### 1

#### 입력

```
4
)(
(()
()
))()
```

#### 출력

```
YES
(())
YES
()()()
NO
YES
()(()())
```

## 티어(난이도)

900

## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 2초 | 256MB |

## 알고리즘 분류

- constructive algorithms
- strings
