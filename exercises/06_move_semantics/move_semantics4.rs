// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec();

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

// 'fill_vec（）' 不再将 'vec： Vec<i32>' 作为参数 - 不要改变这一点！
fn fill_vec() -> Vec<i32> {
    // 相反，让我们在这里创建并填写 Vec - 你是怎么做到的？
    let mut vec = vec![22, 44, 66];

    vec.push(88);

    vec
}