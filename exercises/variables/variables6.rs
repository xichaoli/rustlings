// variables6.rs
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a hint.

// 常量的特点
// 首先，不允许对常量使用 mut。常量不光默认不可变，它总是不可变。
// 声明常量使用 const 关键字而不是 let，
// 并且 必须 注明值的类型。

const NUMBER: i32 = 3;
fn main() {
    println!("Number {}", NUMBER);
}
