// primitive_types2.rs
// Fill in the rest of the line that has code missing!
// No hints, there's no tricks, just get used to typing these :)

// 我们用单引号声明 char 字面量，而与之相反的是，使用双引号声明字符串字面量。
// Rust 的 char 类型的大小为四个字节 (four bytes)，并代表了一个 Unicode 标量值(Unicode Scalar Value)。
// 带变音符号的字母（Accented letters），中文、日文、韩文等字符，emoji 以及零长度的空白字符都是有效的 char 值。
// Unicode 标量值包含从 U+0000 到 U+D7FF 和 U+E000 到 U+10FFFF 在内的值。
// 不过，“字符” 并不是一个 Unicode 中的概念，所以人们直觉上的 “字符” 可能与 Rust 中的 char 并不符合。

fn main() {
    // Characters (`char`)

    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character = '😻';// Finish this line like the example! What's your favorite character?
    // Try a letter, try a number, try a special character, try a character
    // from a different language than your own, try an emoji!
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
