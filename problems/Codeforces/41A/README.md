# [Codeforces 41A : Translation](https://codeforces.com/problemset/problem/41/A)

## 문제 설명

The translation from the Berland language into the Birland language is not an easy task. Those languages are very similar: a Berlandish word differs from a Birlandish word with the same meaning a little: it is spelled (and pronounced) reversely. For example, a Berlandish word code corresponds to a Birlandish word edoc. However, making a mistake during the "translation" is easy. Vasya translated the word s from Berlandish into Birlandish as t. Help him: find out if he translated the word correctly.

## 입력

The first line contains word s, the second line contains word t. The words consist of lowercase Latin letters. The input data do not contain unnecessary spaces. The words are not empty and their lengths do not exceed 100 symbols.

## 출력

If the word t is a word s, written reversely, print YES, otherwise print NO.

## 노트


## 예제

### 1

#### 입력

```
code
edoc
```

#### 출력

```
YES
```

### 2

#### 입력

```
abb
aba
```

#### 출력

```
NO
```

### 3

#### 입력

```
code
code
```

#### 출력

```
NO
```

## 티어(난이도)

800

## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 1초  | 256MB  |

## 알고리즘 분류

- implementation
- strings
