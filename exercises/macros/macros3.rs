// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a hint.

// macro_use属性有两种用途。
// 首先，它可以通过作用于模块的方式让模块内的宏的作用域在模块关闭时不结束;
// 其次，它可以用于从另一个 crate 里来导入宏，方法是将它附加到当前 crate 根模块中的 extern crate 声明前。
// ```
// #[macro_use(lazy_static)] // 或者使用 #[macro_use] 来导入所有宏.
// extern crate lazy_static;
//
// lazy_static!{}
// // self::lazy_static!{} // 报错: lazy_static 没在 `self` 中定义
// ```
// 要用 #[macro_use] 导入宏必须先使用 #[macro_export] 导出. (这句话还没明白)

// 默认情况下，宏没有基于路径的作用域。
// 但是如果该宏带有 #[macro_export] 属性，则相当于它在当前 crate 的根作用域的顶部被声明
//
// 标有 #[macro_export] 的宏始终是 pub 的，
// 以便可以通过路径或前面所述的 #[macro_use] 方式让其他 crate 来引用。

// method one: 让模块内的宏的作用域在模块关闭时不结束
#[macro_use]
mod macros {
    // method two
    // #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
