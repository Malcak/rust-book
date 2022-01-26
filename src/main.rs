fn main() {
    // s1 was invalidated and his value was moved to s2
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);

    // this is how deeply copy the heap data
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}
