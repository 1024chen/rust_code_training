//定义包含引用的结构体（需要为结构体定义中的每一个引用添加生命周期注解）
//类似于泛型参数类型，必须在结构体名称后面的尖括号中声明泛型生命周期参数，以便在结构体定义中使用生命周期参数。
//这个注解意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久。
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
    .next()
    .expect("Could not find a '.'");
    let i = ImportantExcerpt{part:first_sentence};

    println!("{}",i.part);
}
