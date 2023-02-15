// functions4.rs
// Execute `rustlings hint functions4` or use the `hint` watch subcommand for a hint.

// This store is having a sale where if the price is an even number, you get
// 10 Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
// (Don't worry about the function bodies themselves, we're only interested
// in the signatures for now. If anything, this is a good way to peek ahead
// to future exercises!)

// 函数返回值
// 函数可以向调用它的代码返回值。
// 我们并不对返回值命名，但要在箭头（->）后声明它的类型。
// 在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。
// 使用 return 关键字和指定值，可从函数中提前返回；
// 但大部分函数隐式的返回最后的表达式。

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
