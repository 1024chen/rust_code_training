//对应代码清单2-29，切片类型示例
fn main(){
    let arr: [i32;5] = [1,2,3,4,5];
    assert_eq!(&arr,&[1,2,3,4,5]);
    assert_eq!(&arr[1..],[2,3,4,5]);//位置
    //println!("{}",arr.len());//5
    assert_eq!((&arr).len(),5);
    assert_eq!((&arr).is_empty(),false);
    let arr = &mut [1,2,3];
    arr[1] = 7;
    assert_eq!(arr,&[1,7,3]);
    let vec = vec![1,2,3];
    assert_eq!(&vec[..],[1,2,3]);
}