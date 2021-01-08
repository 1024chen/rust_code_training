#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    //当测试函数中出现 panic 时测试就失败了。每一个测试都在一个新线程中运行，当主线程发现测试线程异常了，就将对应测试标记为失败。
    //最简单的造成 panic 的方法：调用 panic! 宏。
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
}
