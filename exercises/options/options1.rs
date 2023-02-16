// options1.rs
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a hint.

// 12AM 是 零点
// 模式匹配语法：
//    使用 | 语法匹配多个模式
//    通过 ..= 匹配值的范围
// unwrap_or(self, default: T) -> T
//   Returns the contained Some value or a provided default.


// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a value of 0
    // The Option output should gracefully handle cases where time_of_day > 23.
    match time_of_day {
        0..=21 => Some(5),
        22 | 23 => Some(0),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams.unwrap_or(0), 5);

        let icecreams1 = maybe_icecream(30);
        assert_eq!(icecreams1.unwrap_or(0), 0);
    }
}
