#[derive(Debug)]
struct Rectangle {
    width: u32,
    height:u32,
}

impl Rectangle {
    fn can_hold(&self,other:&Rectangle) -> bool {
        self.width >other.width && self.height > other.height
    }
}


#[cfg(test)]
mod tests {
    //tests 是一个普通的模块，它遵循第七章 「私有性规则」 部分介绍的可见性规则。因为这是一个内部模块，要测试外部模块中的代码，需要将其引入到内部模块的作用域中。
    //这里选择使用 glob 全局导入，以便在 tests 模块中使用所有在外部模块定义的内容。
    use super::*;

    #[test]
    fn larger_can_hold_smaller()  {
        let larger = Rectangle{
            width:8,
            height:7,
        };
        let smaller = Rectangle{
            width:5,
            height:1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }
}
