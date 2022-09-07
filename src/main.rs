use std::io::stdin;

fn main() {
    let t: i32 = read_line().parse().unwrap();
    for _ in 0..t {
        let n: i32 = read_line().parse().unwrap();
        let mut arr: Vec<i32> = read_line_vector();
        if n == 1 {
            println!("1");
            continue;
        }
        arr.sort();
        let mut concurrent: bool = false;

        for i in 1..n {
            if arr[i as usize] - arr[(i - 1) as usize] == 1 {
                concurrent = true;
                break;
            }
        }

        if concurrent {
            println!("2");
        } else {
            println!("1");
        }
    }
}

fn read_line() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn read_line_vector() -> Vec<i32> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
