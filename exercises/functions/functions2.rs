// functions2.rs
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a hint.

// 在函数签名中，必须 声明每个参数的类型

fn main() {
    call_me(3);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
