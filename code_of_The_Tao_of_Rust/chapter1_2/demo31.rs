//代码清单2-50：Box＜T＞在堆内存中分配值的示例
fn list2_50() {
    #[derive(PartialEq, Debug)]
    struct Point {
        x: f64,
        y: f64,
    }
    let box_point = Box::new(Point { x: 0.0, y: 0.0 });
    let unboxed_point: Point = *box_point;
    assert_eq!(unboxed_point, Point { x: 0.0, y: 0.0 });
}

//代码清单2-52：Option＜T＞应用示例
fn list2_52() {
    use std::fmt::Debug;
    fn match_option<T: Debug>(o: Option<T>) {
        match o {
            Some(i) => println!("{:?}", i),
            None => println!("nothing"),
        }
    }

    let a = Some(3);
    let b = Some("hello");
    let c = Some('A');
    let d: Option<u32> = None;
    match_option(a);
    match_option(b);
    match_option(c);
    match_option(d);
}

//代码清单2-53：trait示例
fn list2_53() {
    struct Duck;
    struct Pig;

    trait Fly {
        fn fly(&self) -> bool;
    }

    impl Fly for Duck {
        fn fly(&self) -> bool {
            true
        }
    }
    impl Fly for Pig {
        fn fly(&self) -> bool {
            false
        }
    }

    fn fly_static<T: Fly>(s: T) -> bool {
        s.fly()
    }

    let pig = Pig;
    assert_eq!(fly_static(pig),false);
    let duck = Duck;
    assert_eq!(fly_static(duck),true);
}

//代码清单2-54：实现Debug trait
fn list2_54() {
    use std::fmt::*;
    struct Point {
        x:i32,
        y:i32,
    }
    impl Debug for Point {
      fn fmt(&self, f: &mut Formatter<'_>) -> Result {
          write!(f,"Point {{ x: {}, y:{} }}",self.x,self.y)
      }  
    }

    let origin = Point {x:0,y:0};
    println!("The origin is: {:?}",origin);
}

//代码清单2-56：Result＜T，E＞使用示例
fn list2_56() {
    let x: Result<i32,&str> = Ok(-3);
    assert_eq!(x.is_ok(),true);
    let x:Result<i32,&str> = Err("Some error message");
    assert_eq!(x.is_ok(),false);
}

use std::fs::File;
fn main() -> Result<(),std::io::Error>{
    list2_50();
    list2_52();
    list2_53();
    list2_54();
    list2_56();

    #[allow(unused)]
    let f = File::open("bar.txt")?;
    Ok(())
}
