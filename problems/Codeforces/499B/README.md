# [Codeforces 499B : Lecture](https://codeforces.com/problemset/problem/499/B)

## 문제 설명

You have a new professor of graph theory and he speaks very quickly. You come up with the following plan to keep up with his lecture and make notes.

You know two languages, and the professor is giving the lecture in the first one. The words in both languages consist of lowercase English characters, each language consists of several words. For each language, all words are distinct, i.e. they are spelled differently. Moreover, the words of these languages have a one-to-one correspondence, that is, for each word in each language, there exists exactly one word in the other language having has the same meaning.

You can write down every word the professor says in either the first language or the second language. Of course, during the lecture you write down each word in the language in which the word is shorter. In case of equal lengths of the corresponding words you prefer the word of the first language.

You are given the text of the lecture the professor is going to read. Find out how the lecture will be recorded in your notes.

## 입력

The first line contains two integers, n and m ($1 \le n \le 3000$, $1 \le m \le 3000$) — the number of words in the professor's lecture and the number of words in each of these languages.

The following m lines contain the words. The i-th line contains two strings $a_i$, $b_i$ meaning that the word $a_i$ belongs to the first language, the word $b_i$ belongs to the second language, and these two words have the same meaning. It is guaranteed that no word occurs in both languages, and each word occurs in its language exactly once.

The next line contains n space-separated strings $c_1, c_2, ... c_n$ — the text of the lecture. It is guaranteed that each of the strings $c_i$ belongs to the set of strings {$a_1, a_2, ... a_m$}.

All the strings in the input are non-empty, each consisting of no more than 10 lowercase English letters.

## 출력

Output exactly n words: how you will record the lecture in your notebook. Output the words of the lecture in the same order as in the input.

## 노트


## 예제

### 1

#### 입력

```
4 3
codeforces codesecrof
contest round
letter message
codeforces contest letter contest
```

#### 출력

```
codeforces round letter round
```

### 2

#### 입력

```
5 3
joll wuqrd
euzf un
hbnyiyc rsoqqveh
hbnyiyc joll joll euzf joll
```

#### 출력

```
hbnyiyc joll joll un joll
```

## 티어(난이도)

1000

## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 1초  | 256MB  |

## 알고리즘 분류

- implementation
- strings