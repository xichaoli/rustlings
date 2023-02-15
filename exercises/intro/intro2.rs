// intro2.rs
// Make the code print a greeting to the world.
// Execute `rustlings hint intro2` or use the `hint` watch subcommand for a hint.


fn main() {
    // solution 1
    let obj = "world";
    // 1-1
    println!("Hello {}", obj);
    // 1-2
    println!("Hello {obj}");

    // solution 3
    // <<rust by example>> 格式化输出 章节
    // 通常情况下，`{}` 会被任意变量内容所替换。
    // 变量内容会转化成字符串。
    // println!("{} days", 31);

    println!("Hello {}!", "world");
}
