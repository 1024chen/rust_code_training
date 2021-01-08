fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    //some_string.push_str(", world");//报错，不可对不可变借用进行修改
}
