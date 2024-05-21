// iterators1.rs
//
// 对集合中的元素执行操作时，迭代器是必不可少的。
// 此模块可帮助您熟悉使用迭代器的结构以及如何遍历可迭代集合中的元素。
//
// 让我通过填写“???的
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.

#[test]
fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter(); // TODO: Step 1

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple")); // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach")); // TODO: Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None); // TODO: Step 4
}