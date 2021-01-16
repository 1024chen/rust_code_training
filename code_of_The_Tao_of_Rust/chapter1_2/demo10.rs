//if表达式，if表达式的分支必须返回同一个类型的值
fn main() {
    let n = 13;//默认推断为i32类型
    let big_n = if (n<10 && n>-10) {
        10 * n
    }else{
        n/2
    };
    assert_eq!(big_n,6);
}