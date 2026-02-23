# [Codeforces 118A : String Task](https://codeforces.com/problemset/problem/118/A)

## 문제 설명

Petya started to attend programming lessons. On the first lesson his task was to write a simple program. The program was supposed to do the following: in the given string, consisting if uppercase and lowercase Latin letters, it:

- deletes all the vowels,
- inserts a character "." before each consonant,
- replaces all uppercase consonants with corresponding lowercase ones.

Vowels are letters "A", "O", "Y", "E", "U", "I", and the rest are consonants. The program's input is exactly one string, it should return the output as a single string, resulting after the program's processing the initial string.

Help Petya cope with this easy task.

## 입력

The first line represents input string of Petya's program. This string only consists of uppercase and lowercase Latin letters and its length is from 1 to 100, inclusive.

## 출력

Print the resulting string. It is guaranteed that this string is not empty.

## 노트


## 예제

### 1

#### 입력

```
tour
```

#### 출력

```
.t.r
```

### 2

#### 입력

```
Codeforces
```

#### 출력

```
.c.d.f.r.c.s
```

### 3

#### 입력

```
aBAcAba
```

#### 출력

```
.b.c.b
```

## 티어(난이도)

1000

## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 2초  | 256MB  |

## 알고리즘 분류

- strings
- implementation