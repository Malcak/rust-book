fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    // cannot borrow `s` as mutable because it is also borrowed as immutable
    
    println!("{}, {}, and {}", r1, r2, r3);
}
