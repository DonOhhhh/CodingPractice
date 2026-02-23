# [Codeforces 1005B : Delete from the Left](https://codeforces.com/problemset/problem/1005/B)

## 문제 설명

You are given two strings s and t. In a single move, you can choose any of two strings and delete the first (that is, the leftmost) character. After a move, the length of the string decreases by 1. You can't choose a string if it is empty.

For example:

- by applying a move to the string "where", the result is the string "here",
- by applying a move to the string "a", the result is an empty string "".

You are required to make two given strings equal using the fewest number of moves. It is possible that, in the end, both strings will be equal to the empty string, and so, are equal to each other. In this case, the answer is obviously the sum of the lengths of the initial strings.

Write a program that finds the minimum number of moves to make two given strings s and t equal.

## 입력

The first line of the input contains s. In the second line of the input contains t. Both strings consist only of lowercase Latin letters. The number of letters in each string is between 1 and $2\cdot10^5$, inclusive.

## 출력

Output the fewest number of moves required. It is possible that, in the end, both strings will be equal to the empty string, and so, are equal to each other. In this case, the answer is obviously the sum of the lengths of the given strings.

## 노트

In the first example, you should apply the move once to the first string and apply the move once to the second string. As a result, both strings will be equal to "est".

In the second example, the move should be applied to the string "codeforces" 8 times. As a result, the string becomes "codeforces" $\to$ "es". The move should be applied to the string "yes" once. The result is the same string "yes" $\to$ "es".

In the third example, you can make the strings equal only by completely deleting them. That is, in the end, both strings will be empty.

In the fourth example, the first character of the second string should be deleted.

## 예제

### 1

#### 입력

```
test
west
```

#### 출력

```
2
```

### 2

#### 입력

```
codeforces
yes
```

#### 출력

```
9
```

### 3

#### 입력

```
test
yes
```

#### 출력

```
7
```

### 4

#### 입력

```
b
ab
```

#### 출력

```
1
```

## 티어(난이도)

900

## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 1초  | 256MB  |

## 알고리즘 분류

- brute force
- implementation
- strings