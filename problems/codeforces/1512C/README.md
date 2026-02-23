# [Codeforces 1512C : A-B Palindrome](https://codeforces.com/problemset/problem/1512/C)

## 문제 설명

You are given a string  $s $ consisting of the characters '$\texttt{0}$', '$\texttt{1}$', and '$\texttt{?}$'. You need to replace all the characters with '$\texttt{?}$' in the string  $s $ by '$\texttt{0}$' or '$\texttt{1}$' so that the string becomes a palindrome and has **exactly**  $a $ characters '$\texttt{0}$' and **exactly**  $b $ characters '$\texttt{1}$'. Note that each of the characters '$\texttt{?}$' is replaced **independently** from the others.

A string  $t $ of length  $n $ is called a palindrome if the equality  $t[i] = t[n-i+1] $ is true for all  $i $ ( $1 \le i \le n $).

For example, if  $s= $"$\texttt{01?????0}$",  $a=4 $ and  $b=4 $, then you can replace the characters '$\texttt{?}$' in the following ways:

- "$\texttt{01011010}$";
- "$\texttt{01100110}$".

For the given string  $s $ and the numbers  $a $ and  $b $, replace all the characters with '$\texttt{?}$' in the string  $s $ by '$\texttt{0}$' or '$\texttt{1}$' so that the string becomes a palindrome and has **exactly**  $a $ characters '$\texttt{0}$' and **exactly**  $b $ characters '$\texttt{1}$'.

## 입력

The first line contains a single integer  $t $ ( $1 \le t \le 10^4 $). Then  $t $ test cases follow.

The first line of each test case contains two integers  $a $ and  $b $ ( $0 \le a, b \le 2 \cdot 10^5 $,  $a + b \ge 1 $).

The second line of each test case contains the string  $s $ of length  $a+b $, consisting of the characters '$\texttt{0}$', '$\texttt{1}$', and '$\texttt{?}$'.

It is guaranteed that the sum of the string lengths of  $s $ over all test cases does not exceed  $2 \cdot 10^5 $.

## 출력

For each test case, output:

- "$\texttt{-1}$", if you can't replace all the characters '$\texttt{?}$' in the string  $s $ by '$\texttt{0}$' or '$\texttt{1}$' so that the string becomes a palindrome and that it contains **exactly**  $a $ characters '$\texttt{0}$' and **exactly**  $b $ characters '$\texttt{1}$';
- the string that is obtained as a result of the replacement, otherwise.

If there are several suitable ways to replace characters, you can output any.

## 노트



## 예제

### 1

#### 입력

```
9
4 4
01?????0
3 3
??????
1 0
?
2 2
0101
2 2
01?0
0 1
0
0 3
1?1
2 2
?00?
4 3
??010?0
```

#### 출력

```
01011010
-1
0
-1
0110
-1
111
1001
0101010
```

## 티어(난이도)

1200

## 제한

| 시간 | 메모리 |
|:----:|:------:|
| 2초 | 256MB |

## 알고리즘 분류

- constructive algorithms
- implementation
- strings
