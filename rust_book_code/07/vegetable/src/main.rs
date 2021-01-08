mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
        pub fn new1(name: &str, id1: i32) -> (Vegetable, i32) {
            (
                Vegetable {
                    name: String::from(name),
                    id: id1,
                },
                id1,
            )
        }
    }
}

fn main() {
    let mut v = plant::Vegetable::new("squash");
    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    let (v, id) = plant::Vegetable::new1("squas", 21);
    println!("this new1 object name : {} ,id: {}", v.name, id);
    // 如果将如下行取消注释代码将无法编译:
    // println!("The ID is {}", v.id);
}
