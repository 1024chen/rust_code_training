fn main() {
    enum Option {
        Some(T),
        None,
    }
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    println!("Hello, world!");
}
