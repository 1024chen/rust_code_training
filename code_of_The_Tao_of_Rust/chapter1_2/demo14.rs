//使用match表达式与while let表达式


//创建动态数组v，调用pop()方法取出并打印
fn main() {
    let mut v = vec![1,2,3,4,5];
    loop {
        //因为调用v.pop()会返回Option类型（该类型用以防止空指针的出现）
        match v.pop() {
            Some(x) => println!("{}",x),//Some(x)为匹配模式,匹配数组中的元素
            None => break,//数组取空的时候，跳出循环
        }
    }
}


/*
//while let 简化表达式
fn main() {
    let mut v = vec![1,2,3,4,5];
    while let Some(x) = v.pop() {//Some(x),匹配右侧pop方法调用返回的Option类型结果，并自动创建x绑定供println!宏语句使用
        println!("{}",x);
    }
}
*/