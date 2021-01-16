fn main(){
    let truth : &'static str = "Rust是一门优雅的语言";
    let ptr = truth.as_ptr();
    let len = truth.len();
    assert_eq!(28,len);
    let s = unsafe {
        let slice = std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8(slice)
    };
    assert_eq!(s,Ok(truth));
}