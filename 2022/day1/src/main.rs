use std::io;

fn main() {
    let mut input = String::new();
    let mut cur = 0;
    let mut totals: Vec<i32> = vec![];
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() {
                    totals.push(cur);
                    cur = 0;
                } else {
                    cur += input.parse::<i32>().unwrap();
                }
            }
            Err(_) => {
                println!("Failed to read input!");
            }
        }
        input.clear();
    }
    totals.sort();
    totals.reverse();
    let ans = totals[0] + totals[1] + totals[2];
    println!("{ans}");
}
