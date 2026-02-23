# [Codeforces 2195A : Sieve of Erato67henes](https://codeforces.com/contest/2195/problem/A)

## 문제 설명

You are given n positive integers $a_1,a_2,\ldots,a_n$.

Please determine if it is possible to select any number of elements in a, so that their product is 67.

Note that you may not select zero elements, as the product of zero elements is not defined in this problem.

## 입력

Each test contains multiple test cases. The first line contains the number of test cases t ($1 \le t \le 10^4$). The description of the test cases follows.

The first line of each test case contains a single integer n ($1 \le n \le 5$).

The second line of each test case contains n positive integers $a_1,a_2,\ldots,a_n (1 \le a_i \le 67)$.

## 출력

If it is possible to select elements so that their product is 67, output "YES" on one line. Otherwise, output "NO" on one line.

You can output the answer in any case. For example, the strings "yEs", "yes", and "Yes" will also be recognized as positive responses.

## 노트

In the first test case, you can select $a_1$ and $a_5$ to get $a_1\cdot a_5 = 1 \cdot 67 = 67$.

In the second test case, it is impossible to select any number of elements so that their product is 67.

## 예제

### 1

#### 입력

```
2
5
1 7 6 7 67
5
1 3 5 7 8
```

#### 출력

```
YES
NO
```

## 티어(난이도)



## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 1초  | 256MB  |

## 알고리즘 분류

