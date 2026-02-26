# [Codeforces 2205B : Simons and Cakes for Success](https://codeforces.com/contest/2205/problem/B)

## 문제 설명

$\mathit{When I succeed, we'll share the cakes together!}$— SHUN

Simons has $n$ friends and a huge amount of cakes. To divide the cakes fairly, you are asked to help him solve the following problem:

 - Find the minimum $\mathbf{positive}$ integer $k$ such that $n$ is a divisor of $k^n$.

It can be proved that the answer always exists under the given constraints.

## 입력

Each test contains multiple test cases. The first line contains the number of test cases $t$ ($1 \le t \le 100$). The description of the test cases follows. 

The only line of each test case contains a single integer $n$ ($2\le n\le 10^9$) — the number of friends Simons has.

## 출력

For each test case, output a single integer — the minimum $k$ you found.

## 노트

In the first test case:

- $1^8=1$, and $8$ is not a divisor of $1$;
- $2^8=256$, and $8$ is a divisor of $256$, because $256 = 8\cdot 32$.

Thus, the minimum possible $k$ is $2$.

In the second test case, $12$ is a divisor of $6^{12}=2\,176\,782\,336$, because $2\,176\,782\,336=12\cdot 181\,398\,528$.

## 예제

### 1

#### 입력

```
4
8
12
369
55635800
```

#### 출력

```
2
6
123
2090
```

## 티어(난이도)

알 수 없음

## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 1초 | 256MB |

## 알고리즘 분류

