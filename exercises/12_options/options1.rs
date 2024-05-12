// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.


// 此函数返回冰箱中还剩下多少冰淇淋。
// 如果在晚上 10 点之前，还剩下 5 勺。晚上10点，有人吃光了，
// 所以不会再有剩下的:(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // 我们在这里使用 24 小时制，因此 10PM 的值为 22,12AM 的值为 0。
    // Option 输出应正常处理time_of_day > 23 的情况。
    // TODO: Complete the function body - remember to return an Option!
    if time_of_day>23 { return None }
    else if time_of_day>=22 { return Some(0); }
    Some(5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams, Some(5));
    }
}