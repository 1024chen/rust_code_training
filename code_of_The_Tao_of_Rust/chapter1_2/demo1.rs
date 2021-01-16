
fn main() {
    let a = [1,2,3];//固长数组
    let b = &a;//取地址
    println!("{:p}",b);//指定打印地址格式
    let mut c = vec![1,2,3];//动长数组
    let d = &mut c;//可变引用，要获取可变引用，必须先声明可变绑定
    d.push(4);//调用push方法插入新元素4
    println!("{:?}",d);
    let e = &42;
    /**
     * 编译器会为&42创建一个临时值，如下：
     * let mut _0: &i32;
     * let mut _1: i32;
     * _1 = const 42i32;
     * _0 = &_1;
     */
    assert_eq!(42,*e);
}