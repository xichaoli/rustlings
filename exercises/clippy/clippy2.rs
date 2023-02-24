// clippy2.rs
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a hint.

// 莫名其妙

fn main() {
    let mut res = 42;
    let option = Some(12);
    // for x in option { // 莫名其妙 的 for
    if let Some(x) = option {
        res += x;
    }
    println!("{res}");
}
