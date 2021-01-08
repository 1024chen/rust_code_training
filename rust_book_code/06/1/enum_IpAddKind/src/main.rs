fn main() {
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    //我们直接将数据附加到枚举的每个成员上，这样就不需要一个额外的结构体了。
    //用枚举替代结构体还有另一个优势：每个成员可以处理不同类型和数量的数据。
    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }

    // let home = IpAddr::V4(String::from("127.0.0.1"));

    // let loopback = IpAddr::V6(String::from("::1"));

    //如果我们想要将 V4 地址存储为四个 u8 值而 V6 地址仍然表现为一个 String，这就不能使用结构体了
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
