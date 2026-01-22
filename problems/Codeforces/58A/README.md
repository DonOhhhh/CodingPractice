# [Codeforces 58A : Chat room](https://codeforces.com/problemset/problem/58/A)

## 문제 설명

Vasya has recently learned to type and log on to the Internet. He immediately entered a chat room and decided to say hello to everybody. Vasya typed the word s. It is considered that Vasya managed to say hello if several letters can be deleted from the typed word so that it resulted in the word "hello". For example, if Vasya types the word "ahhellllloou", it will be considered that he said hello, and if he types "hlelo", it will be considered that Vasya got misunderstood and he didn't manage to say hello. Determine whether Vasya managed to say hello by the given word s.

## 입력

The first and only line contains the word s, which Vasya typed. This word consisits of small Latin letters, its length is no less that 1 and no more than 100 letters.

## 출력

If Vasya managed to say hello, print "YES", otherwise print "NO".

## 노트


## 예제

### 1

#### 입력

```
ahhellllloou
```

#### 출력

```
YES
```

### 2

#### 입력

```
hlelo
```

#### 출력

```
NO
```

## 티어(난이도)

1000

## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 1초  | 256MB  |

## 알고리즘 분류

- strings
- greedy
