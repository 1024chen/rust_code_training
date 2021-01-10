unsafe fn dangerous() {
    println!("dangerous");
}

fn creating_a_safe_abstraction_over_unsafe_code() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn main() {
    unsafe {
        dangerous();
    }

    creating_a_safe_abstraction_over_unsafe_code();
}
