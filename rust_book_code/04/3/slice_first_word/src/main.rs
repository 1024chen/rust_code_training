fn main() {
    let mut s: String = String::from("hello world");

    let word = first_word(&s);
    s.clear();
}

//first_word 函数返回 String 参数的一个字节索引值
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    //用 as_bytes 方法将 String 转化为字节数组,使用 iter 方法在字节数组上创建一个迭代器
    /*
    iter 方法返回集合中的每一个元素，
    而 enumerate 包装了 iter 的结果，将这些元素作为元组的一部分来返回。
    enumerate 返回的元组中，第一个元素是索引，第二个元素是集合中元素的引用
    */

    //通过字节的字面值语法来寻找代表空格的字节。如果找到了一个空格，返回它的位置。
    //否则，使用 s.len() 返回字符串的长度
    for (i, &item) in bytes.iter().enumerate() {
        if item = b' ' {
            return i;
        }
    }
    s.len()
}
