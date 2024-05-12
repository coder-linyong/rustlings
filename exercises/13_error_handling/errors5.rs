// errors5.rs
//
// 该程序使用了errors4 中代码的修改版本。
//
// 本练习使用了一些我们在课程后面才会了解的概念，例如“Box”和“From”特征。
// 现在详细了解它们并不重要，但如果您愿意，可以继续阅读。
// 现在，将 `Box<dyn ???>` 类型视为“我想要任何可以 ??? 的东西”类型，其中，
// 考虑到 Rust 通常的运行时安全标准，你应该会觉得有些宽松！
//
// 简而言之，盒子的这种特殊用例适用于当您想要拥有一个值并且您只关心它是实现特定特征的类型时。
// 为此，将 Box 声明为 Box<dyn Trait> 类型，其中 Trait 是编译器在该上下文中使用的任何值上查找的特征。
// 对于本练习，该上下文是可以在结果中返回的潜在错误。
//
// 我们可以用什么来描述这两个错误？换句话说，是否存在两个错误都实现的特征？
//
// Execute `rustlings hint errors5` or use the `hint` watch subcommand for a
// hint.


use std::error;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
fn main() -> Result<(), Box<dyn Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// 不要更改此线以下的任何内容。

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}