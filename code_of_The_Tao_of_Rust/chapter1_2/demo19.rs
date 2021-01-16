//范围类型
fn main() {
    assert_eq!((1..5),std::ops::Range{start:1,end:5});
    assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5));
    assert_eq!(3 + 4 + 5, (3..6).sum());
    assert_eq!(3 + 4 + 5 + 6, (3..=6).sum());
    for i in (1..5) {
        println!("{}", i);
    }
    for i in (1..=5) {
        println!("{}", i);
    }
}
