// macros1.rs
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a hint.

// 定义宏时 名称后面不加 ！，使用时才加

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
