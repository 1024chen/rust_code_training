use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    //像 vector 一样，哈希 map 将它们的数据储存在堆上，这个 HashMap 的键类型是 String 而值类型是 i32。
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    //另一个构建哈希 map 的方法是使用一个元组的 vector 的 collect 方法，其中每个元组包含一个键值对。
    //collect 方法可以将数据收集进一系列的集合类型，包括 HashMap

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    for item in scores {
        println!("key is {},value is: {}", item.0, item.1);
    }

    //对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。
    //对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //这里field_name和field_value不再有效
    for ite in map {
        println!("key:{},value:{}", ite.0, ite.1);
    }

    //使用vector类似的方式来遍历哈希map中每一个键值对
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //如果我们插入了一个键值对，接着用相同的键插入一个不同的值，与这个键相关联的旧值将被替换
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
    //Entry 的 or_insert 方法在键对应的值存在时就返回这个值的 Entry，如果不存在则将参数作为新值插入并返回修改过的 Entry。
    //这比编写自己的逻辑要简明的多，另外也与借用检查器结合得更好

    //通过哈希 map 储存单词和计数来统计出现次数
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    //or_insert 方法事实上会返回这个键的值的一个可变引用（&mut V）
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    //这里我们将这个可变引用储存在 count 变量中，
    //所以为了赋值必须首先使用星号（*）解引用 count。
    //这个可变引用在 for 循环的结尾离开作用域，这样所有这些改变都是安全的并符合借用规则。
    println!("{:?}", map);
}
