//代码清单2_31：原生指针示例
fn main() {
    let mut x = 10;
    let ptr_x = &mut x as *mut i32;//将&mut x 转换为 *mut i32 原生指针
    let y = Box::new(20);//在堆上创建Box指针
    let ptr_y = &*y as *const i32;//对y指向的堆对象解引用之后再进行引用 后转换成 *const i32类型
    unsafe {
        *ptr_x += *ptr_y;//解引用后相加
    }
    assert_eq!(x, 30);
}
