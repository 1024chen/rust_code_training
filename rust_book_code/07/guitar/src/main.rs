//模块允许我们将代码组织起来
mod sound {
    fn guitar() {
        //函数体
    }
}

mod sound1 {
    mod instrument {
        fn clarient() {
            //函数体
        }
    }
    mod voice {}
}

/**
 * 路径 可以有两种形式：
 * 绝对路径（absolute path）从 crate 根开始，以 crate 名或者字面值 crate 开头。
 * 相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。
 * 绝对路径和相对路径都后跟一个或多个由双冒号（::）分割的标识符。
 */

/**
* Rust 采用模块还有另一个原因：模块是 Rust 中的 私有性边界（privacy boundary）。
如果你希望函数或结构体是私有的，将其放入模块。私有性规则有如下：
所有项（函数、方法、结构体、枚举、模块和常量）默认是私有的。
可以使用 pub 关键字使项变为公有。
不允许使用定义于当前模块的子模块中的私有代码。
允许使用任何定义于父模块或当前模块中的代码。
*/
fn main() {
    //绝对路径
    crate::sound1::instrument::clarient();
    //相对路径
    sound::guitar();
}
