//代码清单2-44：Vec＜T＞示例
fn list2_44() {
    let mut v1 = vec![];
    v1.push(1);
    v1.push(2);
    v1.push(3);
    assert_eq!(v1, [1, 2, 3]);
    assert_eq!(v1[1], 2);
    let mut v2 = vec![0; 10];
    let mut v3 = Vec::new();
    v3.push(4);
    v3.push(5);
    v3.push(6);
    v2[0] = 10;
}

//代码清单2-45：可增长双端队列VecDeque＜T＞示例
fn list2_45() {
    use std::collections::VecDeque;

    let mut buf = VecDeque::new();
    buf.push_front(1);
    buf.push_front(2);
    assert_eq!(buf.get(0), Some(&2));
    assert_eq!(buf.get(1), Some(&1));
    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);
    assert_eq!(buf.get(2), Some(&3));
    assert_eq!(buf.get(3), Some(&4));
    assert_eq!(buf.get(4), Some(&5));
}

//代码清单2-46：LinkedList＜T＞示例
fn list2_46() {
    use std::collections::LinkedList;

    let mut list1 = LinkedList::new();
    list1.push_back('a');
    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');
    list1.append(&mut list2);
    println!("{:?}", list1);
    println!("{:?}", list2);
    assert_eq!(list1.pop_back(), Some('c'));
    println!("after pop_back(): {:?}", list1);
    assert_eq!(list1.pop_front(), Some('a'));
    println!("after pop_front(): {:?}", list1);
    list1.push_front('e');
    println!("after push_front() list1: {:?}", list1);
}

//代码清单2-47：HashMap＜K，V＞和BTreeMap＜K，V＞示例
fn list2_47() {
    use std::collections::BTreeMap;
    use std::collections::HashMap;

    let mut hmap = HashMap::new();
    let mut bump = BTreeMap::new();
    hmap.insert(3, "c");
    hmap.insert(1, "a");
    hmap.insert(2, "b");
    hmap.insert(5, "e");
    hmap.insert(4, "d");
    bump.insert(3, "c");
    bump.insert(2, "b");
    bump.insert(1, "a");
    bump.insert(5, "e");
    bump.insert(4, "d");
    println!("{:?}", hmap);
    println!("{:?}", bump);
}

//代码清单2-48：HashSet＜K＞和BTreeSet＜K＞示例
fn list2_48() {
    use std::collections::BTreeSet;
    use std::collections::HashSet;
    let mut hbooks = HashSet::new();
    let mut bbooks = BTreeSet::new();
    hbooks.insert("A Song of Ice and Fire");
    hbooks.insert("The Emerald City");
    hbooks.insert("The Odyssey");

    if !hbooks.contains("The Emerald City") {
        println!(
            "We have {} books,but The Emerald City ain't one.",
            hbooks.len()
        );
    }
    println!("{:?}", hbooks);
    bbooks.insert("A Song of Ice and Fire");
    bbooks.insert("The Emerald City");
    bbooks.insert("The Odyssey");
    println!("{:?}", bbooks);
}

//代码清单2-49：BinaryHeap＜T＞示例
fn list2_49() {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    assert_eq!(heap.peek(),None);
    let arr = [93,80,22,45,78,64,15,35,45,30,22];
    for &i in arr.iter() {
        heap.push(i);
    }
    assert_eq!(heap.peek(),Some(&93));
    println!("{:?}",heap);
}

fn main() {
    list2_44();
    list2_45();
    list2_46();
    list2_47();
    list2_48();
    list2_49();
}
