use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Codeforces,
    BOJ,
    LeetCode,
}

impl Category {
    // 모든 용도(디렉토리명, 쿠키파일명, 로그 등)의 표준 문자열
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Codeforces => "codeforces",
            Category::BOJ => "acmicpc",
            Category::LeetCode => "leetcode",
        }
    }

    pub fn cookie_path(&self) -> String {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home/codefreak".to_string());
        format!("{}/CodingPractice/runner/resources/cookies/{}.json", home, self.as_str())
    }
}

// Display를 구현하면 {} 만으로 문자열 출력이 가능합니다.
impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
