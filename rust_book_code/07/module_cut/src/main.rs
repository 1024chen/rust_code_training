mod sound;

fn main() {
    //绝对路径
    crate::sound::insturment::clarinet();
    //相对路径
    sound::insturment::clarinet();
}
