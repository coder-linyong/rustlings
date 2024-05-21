// tests2.rs
//
// 这个测试有个问题 -- 让测试编译！测试通过！
// 让测试失败！
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(true, true);
    }
}