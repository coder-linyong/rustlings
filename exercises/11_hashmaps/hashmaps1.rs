// hashmaps1.rs
//
// 需要定义一篮子哈希地图形式的水果。键表示水果的名称，值表示购物篮中有多少个特定水果。
// 您必须在篮子中放入至少三种不同类型的水果（例如苹果、香蕉、芒果），并且所有水果的总数量应至少为五种。
//
// 让我编译并通过测试！
//
// 执行 'rustlings hint hashmaps1' 或使用 'hint' watch 子命令获取提示。

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new(); // TODO: declare your hash map here.

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket here.
    basket.insert(String::from("banana1"), 2);
    basket.insert(String::from("banana2"), 2);
    basket.insert(String::from("banana3"), 2);
    basket.insert(String::from("banana4"), 2);

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}