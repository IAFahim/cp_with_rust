#[allow(unused_assignments)]
#[allow(unused_variables)]
fn solve<R: std::io::BufRead, W: std::io::Write>(sc: &mut Scanner<R>, wr: &mut W) {
    
}

fn main() {
    let mut sc = Scanner::new(std::io::stdin().lock());
    match std::fs::File::open("input.txt") {
        Ok(file) => {
            let mut sc = Scanner::new(std::io::BufReader::new(file));
            let mut wr = std::io::BufWriter::new(std::io::stdout());
            match std::fs::File::create("output.txt") {
               Ok(file) => {
                   let mut wr = std::io::BufWriter::new(file);
                   solve(&mut sc, &mut wr);
               }Err(_) => {}
            }
            solve(&mut sc, &mut wr);
        }
        Err(_) => {}
    }
    let stdout = std::io::stdout();
    let mut wr = std::io::BufWriter::new(stdout.lock());
    solve(&mut sc, &mut wr);
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
