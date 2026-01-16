# [Codeforces 112A : Petya and Strings](https://codeforces.com/problemset/problem/112/A)

## 문제 설명

Little Petya loves presents. His mum bought him two strings of the same size for his birthday. The strings consist of uppercase and lowercase Latin letters. Now Petya wants to compare those two strings lexicographically. The letters' case does not matter, that is an uppercase letter is considered equivalent to the corresponding lowercase letter. Help Petya perform the comparison.

## 입력

Each of the first two lines contains a bought string. The strings' lengths range from 1 to 100 inclusive. It is guaranteed that the strings are of the same length and also consist of uppercase and lowercase Latin letters.

## 출력

If the first string is less than the second one, print "-1". If the second string is less than the first one, print "1". If the strings are equal, print "0". Note that the letters' case is not taken into consideration when the strings are compared.

## 예제

### 1

#### 입력

```bash
aaaa
aaaA
```

#### 출력

```bash
0
```

### 2

#### 입력

```bash
abs
Abz
```

#### 출력

```bash
-1
```

### 3

#### 입력

```bash
abcdefg
AbCdEfF
```

#### 출력

```bash
1
```

## 티어(난이도)

800

## 제한

| 시간 | 메모리 |
| ---- | ------ |
| 2초  | 256MB  |

## 알고리즘 분류

- 구현
- 문자열