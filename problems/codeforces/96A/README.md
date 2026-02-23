# [Codeforces 96A : Football](https://codeforces.com/problemset/problem/96/A)

## 문제 설명

Petya loves football very much. One day, as he was watching a football match, he was writing the players' current positions on a piece of paper. To simplify the situation he depicted it as a string consisting of zeroes and ones. A zero corresponds to players of one team; a one corresponds to players of another team. If there are at least 7 players of some team standing one after another, then the situation is considered dangerous. For example, the situation 00100110111111101 is dangerous and 11110111011101 is not. You are given the current situation. Determine whether it is dangerous or not.

## 입력

The first input line contains a non-empty string consisting of characters "0" and "1", which represents players. The length of the string does not exceed 100 characters. There's at least one player from each team present on the field.

## 출력

Print "YES" if the situation is dangerous. Otherwise, print "NO".

## 노트


## 예제

### 1

#### 입력

```
001001
```

#### 출력

```
NO
```

### 2

#### 입력

```
1000000001
```

#### 출력

```
YES
```

## 티어(난이도)

900

## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 2초  | 256MB  |

## 알고리즘 분류

- implementation
- strings