// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.

// 函数的返回值等同于函数体最后一个表达式的值

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
