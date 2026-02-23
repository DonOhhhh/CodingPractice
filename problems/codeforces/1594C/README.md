# [Codeforces 1594C : Make Them Equal](https://codeforces.com/problemset/problem/1594/C)

## 문제 설명

Theofanis has a string $s_1 s_2 \dots s_n$ and a character c. He wants to make all characters of the string equal to c using the minimum number of operations.

In one operation he can choose a number x ($1 \le x \le n$) and for every position i, where i is not divisible by x, replace $s_i$ with c.

Find the minimum number of operations required to make all the characters equal to c and the x-s that he should use in his operations.

## 입력

The first line contains a single integer t ($1 \le t \le 10^4$) — the number of test cases.

The first line of each test case contains the integer n ($3 \le n \le 3 \cdot 10^5$) and a lowercase Latin letter c — the length of the string s and the character the resulting string should consist of.

The second line of each test case contains a string s of lowercase Latin letters — the initial string.

It is guaranteed that the sum of n over all test cases does not exceed $3 \cdot 10^5$.

## 출력

For each test case, firstly print one integer m — the minimum number of operations required to make all the characters equal to c.

Next, print m integers $x_1, x_2, \dots, x_m (1 \le x_j \le n)$ — the x-s that should be used in the order they are given.

It can be proved that under given constraints, an answer always exists. If there are multiple answers, print any.

## 노트

1. 첫 번째 예제: 4 a / aaaa

- 상황: 4명이 있고, 모두 'a'로 만들어야 함. 현재 상태는 aaaa.
- 분석: 이미 모든 사람이 'a' 옷을 입고 있습니다.
- 결과: 할 일이 없습니다.
- 출력: 0

2. 두 번째 예제: 4 a / baaa

- 상황: 4명이 있고, 모두 'a'로 만들어야 함. 현재 상태는 baaa (1번이 'b').
- 전략: 1회 만에 끝낼 수 있는 번호($x$)가 있는지 찾아봅니다.
    - $x=2$ 선택: 2의 배수는 2번, 4번입니다. 둘 다 이미 'a'인가요? 네! (baaa에서 2, 4번째는 'a'입니다.)
    - $x=2$를 외치면 2, 4번은 가만히 있고 나머지(1, 3번)가 'a'로 갈아입습니다.
    - $x=3$ 선택: 3의 배수는 3번뿐입니다. 이미 'a'인가요? 네!
    - $x=3$을 외치면 3번은 가만히 있고 나머지가 'a'로 갈아입습니다.
    - $x=4$ 선택: 4의 배수는 4번뿐입니다. 이미 'a'인가요? 네!
- 결과: $2, 3, 4$ 중 아무거나 골라도 1번 만에 끝납니다. (예제 출력은 $x=2$를 선택한 경우네요.)
- 출력: 1 (연산 횟수) / 2 (선택한 $x$)

3. 세 번째 예제: 4 b / bzyx

- 상황: 4명이 있고, 모두 'b'로 만들어야 함. 현재 상태는 bzyx.
- 1회 도전: 1번 만에 끝낼 수 있는 $x$가 있는지 검사합니다.
    - $x=1$: 1, 2, 3, 4번 모두 'b'여야 함 (실패)
    - $x=2$: 2, 4번이 'b'여야 함 (실패, 2번 'z', 4번 'x')
    - $x=3$: 3번이 'b'여야 함 (실패, 3번 'y')
    - $x=4$: 4번이 'b'여야 함 (실패, 4번 'x')
- 전략: 한 번에 끝낼 $x$가 전혀 없습니다. 이럴 땐 고민 없이 2회 전략을 씁니다.
    - 1회차 (x=2): 2의 배수인 2, 4번만 빼고 다 'b'로 바꿈 $\rightarrow$ bzbx가 됨.
    - 2회차 (x=3): 3번 빼고 다 'b'로 바꿈 $\rightarrow$ 2, 4번 자리가 'b'로 바뀌며 bbbb 완성!
- 결과: 2번의 연산이 필요합니다. 
- 출력: 2 (연산 횟수) / 2 3 (선택한 $x$들)

## 예제

### 1

#### 입력

```
3
4 a
aaaa
4 a
baaa
4 b
bzyx
```

#### 출력

```
0
1
2
2
2 3
```

## 티어(난이도)

1200

## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 2초  | 256MB  |

## 알고리즘 분류

- brute force
- greedy
- math
- strings