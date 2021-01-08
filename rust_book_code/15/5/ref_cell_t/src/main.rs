use std::cell::RefCell;
use std::rc::Rc;

//RefCell<T> 正是用于当你确信代码遵守借用规则，而编译器不能理解和确定的时候。
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil))); //将列表 a 封装进了 Rc<T> 这样当创建列表 b 和 c 时，他们都可以引用 a

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
//标准库中也有其他提供内部可变性的类型，比如 Cell<T>，它类似 RefCell<T> 但有一点除外：它并非提供内部值的引用，而是把值拷贝进和拷贝出 Cell<T>
