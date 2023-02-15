// variables4.rs
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a hint.

// rust中变量默认不可变，使用mut定义可变变量

fn main() {
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
