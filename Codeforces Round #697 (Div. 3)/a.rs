#[allow(unused_assignments)]
#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(sc: &mut Scanner<R>, wr: &mut W) {
    let t: u32 = sc.next();
    for _ in 0..t {
        let n: i64 = sc.next();
        if n & (n - 1) != 0 {
            writeln!(wr, "YES").unwrap();
        } else {
            writeln!(wr, "NO").unwrap();
        }
    }
}

fn main() {
    match std::fs::File::open("C:\\Users\\Fahim\\Rusty\\cp_with_rust\\src\\input.txt") {
        Ok(file) => {
            let mut sc = Scanner::new(std::io::BufReader::new(file));
            match std::fs::File::create("C:\\Users\\Fahim\\Rusty\\cp_with_rust\\src\\output.txt") {
                Ok(file) => {
                    solve(&mut sc, &mut std::io::BufWriter::new(file));
                }
                Err(_) => {
                    solve(&mut sc, &mut std::io::BufWriter::new(std::io::stdout()));
                }
            }
        }
        Err(_) => {
            solve(
                &mut Scanner::new(std::io::stdin().lock()),
                &mut std::io::BufWriter::new(std::io::stdout().lock()),
            );
        }
    }
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
    fn next<T: std::str::FromStr>(&mut self) -> T {
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
