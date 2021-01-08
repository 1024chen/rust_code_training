// 整体文档，位于项目首页
//!  # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain calculations more convenient.

// 文档注释，位于函数前置
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// //在文档注释中增加示例代码块是一个清楚的表明如何使用库的方法，
/// //这么做还有一个额外的好处：cargo test 也会像测试那样运行文档中的示例代码
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub mod kinds {
    use std::fmt;
    //use std::cmp::PartialEq;
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }

    impl PartialEq for PrimaryColor {
        fn eq(&self, other: &PrimaryColor) -> bool {
            let jude = match (&self, other) {
                (PrimaryColor::Red, PrimaryColor::Red) => true,
                (PrimaryColor::Yellow, PrimaryColor::Yellow) => true,
                (PrimaryColor::Blue, PrimaryColor::Blue) => true,
                _ => false,
            };
            jude
        }
    }

    impl PartialEq for SecondaryColor {
        fn eq(&self, other: &SecondaryColor) -> bool {
            let jude = match (&self, other) {
                (SecondaryColor::Green, SecondaryColor::Green) => true,
                (SecondaryColor::Orange, SecondaryColor::Orange) => true,
                (SecondaryColor::Purple, SecondaryColor::Purple) => true,
                _ => false,
            };
            jude
        }
    }

    impl fmt::Debug for SecondaryColor {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SecondaryColor")
                .field("SecondaryColor", &self)
                .finish()
        }
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        if c1 == PrimaryColor::Blue && c2 == PrimaryColor::Yellow {
            return SecondaryColor::Green;
        }
        SecondaryColor::Purple
    }

    #[test]
    fn test_mix() {
        assert_eq!(
            mix(PrimaryColor::Blue, PrimaryColor::Yellow),
            SecondaryColor::Green
        );
    }
}
