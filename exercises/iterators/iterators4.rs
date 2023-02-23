// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.

// fn fold<B, F>(self, init: B, f: F) -> B
// where
//     F: FnMut(B, Self::Item) -> B,
// 通过应用操作将每个元素 fold 到一个累加器中，返回最终结果。
// fold() 有两个参数：一个初始值，一个闭包，有两个参数：一个 ‘accumulator’ 和一个元素。
// 闭包返回累加器在下一次迭代中应具有的值。

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    match num {
        0 => 1,
        _ => {
            (1..=num).fold(1, |acc, x| acc * x)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
