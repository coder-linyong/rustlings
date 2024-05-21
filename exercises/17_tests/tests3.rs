// tests3.rs
//
// 这个测试不是在测试我们的功能——让它以测试通过的方式进行测试。
// 然后写第二个测试来测试我们是否得到了我们调用“is_even（5）”时期望得到的结果。
//
// Execute `rustlings hint tests3` or use the `hint` watch subcommand for a
// hint.

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2));
    }

    #[test]
    fn is_false_when_odd() {
        assert_eq!(is_even(5), false)
    }
}