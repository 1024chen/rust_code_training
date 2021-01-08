//在 mod instrument 和 fn clarinet 之前都增加 pub 关键字使我们可以在 main 中调用此函数
mod sound{
    pub mod instrument{
        pub fn clarient(){
            super::breathe_in();
            println!("clarient.");
        }
    }

    fn breathe_in(){
        println!("breathe_in.");
    }
}

//sound 模块不是公有的，
//不过因为 main 函数与 sound 定义于同一模块（crate）中，
//可以从 main 中引用 sound
//接下来是 instrument，这个模块标记为 pub。
//我们可以访问 instrument 的父模块，所以可以访问 instrument。
//最后，clarinet 函数被标记为 pub 所以可以访问其父模块，所以这个函数调用是有效的！

fn main() {
    //绝对路径
    crate::sound::instrument::clarient();

    //相对路径
    sound::instrument::clarient();
}
