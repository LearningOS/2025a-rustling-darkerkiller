// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

// clippy2.rs
fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {   // 替代 for
        res += x;
    }
    println!("{}", res);
}
