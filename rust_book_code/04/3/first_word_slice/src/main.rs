fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // first_word 中传入 `String` 的 slice
    let word = first_word(&my_string[..]);
    println!("word:{}", word);

    //my_string.clear();//cannot borrow `my_string` as mutable, as it is not declared as mutable
    //上面出现的是不可变引用，这里出现可变引用自然会冲突(当拥有某值的不可变引用时，就不能再获取一个可变引用)

    let my_string_literal = "hello world";

    // first_word 中传入字符串字面值的 slice
    let word = first_word(&my_string_literal[..]);
    println!("word:{}", word);
    // 因为字符串字面值就是字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word(my_string_literal);
    println!("word:{}", word);
}
