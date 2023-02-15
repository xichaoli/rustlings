// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.

// 一个 Message 枚举，其每个成员都存储了不同数量和类型的值

#[derive(Debug)]
enum Message {
    Move { x: i32, y: i32},
    Echo (String),
    ChangeColor(u8, u8, u8),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
