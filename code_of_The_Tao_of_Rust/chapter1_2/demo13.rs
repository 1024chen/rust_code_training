//if let表达式
fn main() {
    let boolean = true;
    let mut binary = 0;
    //如果boolean为true，则将binary的值改为1
    //这里如果将判断条件反序，会导致大量警告
    if let true = boolean {
        binary = 1;
    }
    assert_eq!(binary, 1);
}