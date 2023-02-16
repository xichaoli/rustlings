// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

// 使用 ref 关键字，该值仅被借用，而不移动，从而使它可在 match 语句之后使用

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
