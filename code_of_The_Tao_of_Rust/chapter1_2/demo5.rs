//函数作为返回值
fn is_true() -> bool { true }
fn true_maker() -> fn() -> bool { is_true}
fn main() {
    assert_eq!(true_maker()(),true);
    //true_maker()()相当于(true_maker())()
    //调用true_maker(),会返回is_true函数指针；然后再调用is_true()函数，最终得到true
    
}