#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(scanner: &mut Scanner<R>, writer: &mut W) {
    let n: i32 = scanner.scan();
    for _ in 0..n {
        let a: i32 = scanner.scan();
        let b: i32 = scanner.scan();
        let n: i32 = scanner.scan();
        let s: i32 = scanner.scan();
        let mut ans = "NO";

        if s - a.min(s / n) * n <= b {
            ans = "YES";
        }

        writeln!(writer, "{}", ans).unwrap();
    }
}

fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let mut writer = std::io::BufWriter::new(stdout.lock());
    let mut scanner = Scanner::new(stdin.lock());
    solve(&mut scanner, &mut writer);
}

#[allow(dead_code)]
struct Scanner<R> {
    reader: R,
    line: Vec<u8>,
    ptr: usize,
}

impl<R: std::io::BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        return Self {
            reader,
            line: vec![],
            ptr: 0,
        };
    }
    #[allow(dead_code)]
    fn scan<T: std::str::FromStr>(&mut self) -> T {
        loop {
            while self.ptr < self.line.len() && self.line[self.ptr].is_ascii_whitespace() {
                self.ptr += 1;
            }
            if self.ptr != self.line.len() {
                let start = self.ptr;
                while self.ptr < self.line.len() && !self.line[self.ptr].is_ascii_whitespace() {
                    self.ptr += 1;
                }
                return std::str::from_utf8(&self.line[start..self.ptr])
                    .unwrap()
                    .parse()
                    .ok()
                    .expect("parse error");
            }
            self.line.clear();
            self.reader
                .read_until(b'\n', &mut self.line)
                .expect("read error");
            self.ptr = 0;
        }
    }
}
