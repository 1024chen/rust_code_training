use std::thread;
use std::time::Duration;

fn main() {
    let handle =  thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    //handle.join().unwrap(); 
    //调用 handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束。阻塞（Blocking） 线程意味着阻止该线程执行工作或退出。因为我们将 join 调用放在了主线程的 for 循环之后

    for i in 1..5 {
        println!("hi number {} from the main thread!",i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();//从 thread::spawn 保存一个 JoinHandle 以确保该线程能够运行至结束
}
