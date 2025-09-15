// traits5.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits5` or use the `hint` watch subcommand for a
// hint.

// traits5.rs

pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

// 1. 超级 trait
trait CombinedTrait: SomeTrait + OtherTrait {}
impl<T: SomeTrait + OtherTrait> CombinedTrait for T {}

struct SomeStruct {}
struct OtherStruct {}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// 2. 使用超级 trait object
fn some_func(item: &dyn CombinedTrait) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    some_func(&SomeStruct {});
    some_func(&OtherStruct {});
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_other() {
        assert!(some_func(&SomeStruct {}));
        assert!(some_func(&OtherStruct {}));
    }
}
