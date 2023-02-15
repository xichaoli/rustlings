// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.

// 调用函数时，参数的个数和类型要和声明函数定义时一致

fn main() {
    call_me(3);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
