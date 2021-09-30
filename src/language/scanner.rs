use std::str::Chars;

pub struct Scanner<'a> {
    start: Chars<'a>,
    current: Chars<'a>,
    line: i32,
}

impl Scanner<'_> {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            start: source.chars(),
            current: source.chars(),
            line: 1
        }
    }
}