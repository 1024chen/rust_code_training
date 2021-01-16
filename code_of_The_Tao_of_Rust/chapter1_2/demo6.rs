//const fn示例
const fn init_len() -> usize {
    return 5;
}
fn main() {
    let arr = [0,init_len()];//[0,N]形式初始化初始值为0，长度为N的数组，N由调用函数init_len求得
}