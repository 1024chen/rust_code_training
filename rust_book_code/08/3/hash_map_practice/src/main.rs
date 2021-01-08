use std::collections::HashMap;

fn main() {
    //给定一系列数字，使用 vector 并返回这个列表的平均数（mean, average）、
    //中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希函数会很有帮助）。

    let mut list = vec![8, 9, 3, 6, 7, 6, 5, 2, 4, 1, 6];
    let mut freq: i32 = 0; //计数器
    let mut sum: i32 = 0; //总
    let mut mode_num: i32 = 0; //众数次数
    let mut mode: i32 = 0; //众数

    let mut map = HashMap::new();
    for item in list {
        freq += 1;
        sum += item;
        let count = map.entry(item).or_insert(0);
        *count += 1;
        if mode_num < *count {
            mode_num = *count;
            mode = item;
        }
    }

    let average = sum / freq;

    println!("{:?}", map);
    println!("众数是：{}", mode);
    println!("均数是：{}", average);
}

fn sort(list: &Vec<i32>) {
    let len = list.len();
    for item in list {}
}
