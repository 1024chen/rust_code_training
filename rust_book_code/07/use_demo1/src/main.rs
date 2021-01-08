mod sound{
    pub mod instrument{
        pub fn clarinet(){
            println!("clarient");
        }
    }
}

mod performance_group {
    //use crate::sound::instrument;
    use super::sound::instrument;
    pub fn clarient_trio(){
        instrument::clarinet();
        println!("clarient_trio");
    }
}

//当指定 use 后以 self 开头的相对路径在未来可能不是必须的；这是一个开发者正在尽力消除的语言中的不一致。
use self::sound::instrument;

fn main() {
    instrument::clarinet();
    performance_group::clarient_trio();
}
