// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

// 数组
// 定义后长度就固定了
// 所以成员类型相同
// 计算成员个数使用 len() 方法
// a.iter() 进行引用迭代

fn main() {
    let a = [0;100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
