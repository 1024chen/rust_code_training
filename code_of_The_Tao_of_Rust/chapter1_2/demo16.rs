//基本数字类型
fn main() {
    let num = 42u32;
    let num : u32 = 42;
    let num = 0x2A;//十六进制
    let num = 0o106;//八进制
    let num = 0b1101_1011;//二进制
    assert_eq!(b'*',42u8);//字节字面量
    assert_eq!(b'\'',39u8);
    let num = 3.1415926f64;
    assert_eq!(-3.14,-3.14f64);
    assert_eq!(2., 2.0f64);
    assert_eq!(2e4, 20000f64);
    println!("{:?}", std::f32::INFINITY);//inf
    println!("{:?}", std::f32::NEG_INFINITY);//-inf
    println!("{:?}", std::f32::NAN);//NAN
    println!("{:?}", std::f32::MIN);//-340282350000000000000000000000000000000.0
    println!("{:?}", std::f32::MAX);//340282350000000000000000000000000000000.0

}