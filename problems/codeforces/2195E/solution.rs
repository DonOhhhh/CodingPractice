use std::io::{self, BufWriter, Read, Write};

const MOD: i64 = 1_000_000_007;

// 입력을 빠르게 받기 위한 매크로
macro_rules! scan {
    ($it: expr, $t: ty) => {
        $it.next().unwrap().parse::<$t>().unwrap()
    };
}

struct Node {
    l: usize,
    r: usize,
}

fn solve() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let mut iter = input.split_ascii_whitespace();
    let mut writer = BufWriter::new(io::stdout().lock());

    let t = scan!(iter, usize);

    for _ in 0..t {
        let n = scan!(iter, usize);
        
        // 1-based index 사용 (0번은 더미 혹은 루트의 부모)
        let mut nodes = Vec::with_capacity(n + 1);
        // 0번 인덱스 채우기 (사용 안 함)
        nodes.push(Node { l: 0, r: 0 });

        for _ in 1..=n {
            let l = scan!(iter, usize);
            let r = scan!(iter, usize);
            nodes.push(Node { l, r });
        }

        // 1. 방문 순서 기록 (BFS/DFS를 통해 위상 정렬 순서 확보)
        // 부모 -> 자식 순서 (Pre-order)
        let mut order = Vec::with_capacity(n);
        let mut stack = vec![1]; // 1번 노드부터 시작

        while let Some(u) = stack.pop() {
            order.push(u);
            let l = nodes[u].l;
            let r = nodes[u].r;
            // 스택이므로 오른쪽을 먼저 넣어야 왼쪽이 먼저 나옴 (순서는 크게 상관 없음)
            if r != 0 { stack.push(r); }
            if l != 0 { stack.push(l); }
        }

        // 2. Escape Time (E) 계산 : Bottom-Up (역순 순회)
        let mut dp_e = vec![0i64; n + 1];
        
        // order를 거꾸로 돌면 자식들이 먼저 처리됨
        for &u in order.iter().rev() {
            let l = nodes[u].l;
            let r = nodes[u].r;

            if l == 0 && r == 0 {
                // 리프 노드
                dp_e[u] = 1;
            } else {
                // 내부 노드: E = E_L + E_R + 3
                let e_l = dp_e[l];
                let e_r = dp_e[r];
                dp_e[u] = (e_l + e_r + 3) % MOD;
            }
        }

        // 3. Answer Time (Ans) 계산 : Top-Down (정순 순회)
        let mut ans = vec![0i64; n + 1];

        // 루트의 자식(1번)은 탈출 시간과 동일
        ans[1] = dp_e[1];

        for &u in order.iter() {
            let l = nodes[u].l;
            let r = nodes[u].r;

            if l != 0 { // 자식이 있다면 (Full Binary Tree이므로 둘 다 있음)
                // 공식: Ans_Child = Ans_Parent + E_Child
                ans[l] = (ans[u] + dp_e[l]) % MOD;
                ans[r] = (ans[u] + dp_e[r]) % MOD;
            }
        }

        // 출력
        for i in 1..=n {
            write!(writer, "{} ", ans[i])?;
        }
        writeln!(writer)?;
    }

    Ok(())
}

fn main() {
    solve().unwrap();
}