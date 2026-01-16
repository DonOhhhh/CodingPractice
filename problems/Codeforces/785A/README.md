# [Codeforces 785A : Anton and Polyhedrons](https://codeforces.com/problemset/problem/785/A)

## 문제 설명

Anton's favourite geometric figures are regular polyhedrons. Note that there are five kinds of regular polyhedrons:

- Tetrahedron. Tetrahedron has 4 triangular faces.
- Cube. Cube has 6 square faces.
- Octahedron. Octahedron has 8 triangular faces.
- Dodecahedron. Dodecahedron has 12 pentagonal faces.
- Icosahedron. Icosahedron has 20 triangular faces.

All five kinds of polyhedrons are shown on the picture below:

![alt text](image.png)

Anton has a collection of n polyhedrons. One day he decided to know, how many faces his polyhedrons have in total. Help Anton and find this number!

## 입력

The first line of the input contains a single integer n (1 ≤ n ≤ 200 000) — the number of polyhedrons in Anton's collection.

Each of the following n lines of the input contains a string si — the name of the i-th polyhedron in Anton's collection. The string can look like this:

- "Tetrahedron" (without quotes), if the i-th polyhedron in Anton's collection is a tetrahedron.
- "Cube" (without quotes), if the i-th polyhedron in Anton's collection is a cube.
- "Octahedron" (without quotes), if the i-th polyhedron in Anton's collection is an octahedron.
- "Dodecahedron" (without quotes), if the i-th polyhedron in Anton's collection is a dodecahedron.
- "Icosahedron" (without quotes), if the i-th polyhedron in Anton's collection is an icosahedron.

## 출력

Output one number — the total number of faces in all the polyhedrons in Anton's collection.

## 노트

In the first sample Anton has one icosahedron, one cube, one tetrahedron and one dodecahedron. Icosahedron has 20 faces, cube has 6 faces, tetrahedron has 4 faces and dodecahedron has 12 faces. In total, they have 20 + 6 + 4 + 12 = 42 faces.

## 예제

### 1

#### 입력

```bash
4
Icosahedron
Cube
Tetrahedron
Dodecahedron

```

#### 출력

```bash
42
```

### 2

#### 입력

```bash
3
Dodecahedron
Octahedron
Octahedron

```

#### 출력

```bash
28

```


## 티어(난이도)

800

## 제한

| 시간 | 메모리 |
| ---- | ------ |
| 1초  | 256MB  |

## 알고리즘 분류

- implementation
- strings