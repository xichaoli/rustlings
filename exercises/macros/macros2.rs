// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.

// 与 Rust 中的其他内容不同，“定义宏的位置”与“使用它的位置”的顺序实际上很重要。

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
