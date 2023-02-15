// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a hint.

// str -> String

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    // method 1:
    String::from("blue")

    // method 2:
    // "blue".to_string()

    // method 3:
    // "blue".to_owned()

    // method 4:
    // "blue".into()
}
