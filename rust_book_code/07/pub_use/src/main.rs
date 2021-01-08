//重导出
mod sound{
    pub mod instrument{
        pub fn clarient(){
            println!("clarient");
        }
    }
}

mod perfromance_group{
    pub use crate::sound::instrument;

    pub fn clarient_trio(){
        instrument::clarient();
        println!("clarient_trio");
    }
}

fn main() {
    perfromance_group::clarient_trio();
    perfromance_group::instrument::clarient();
}
