use std::io;

fn main() {
    let mut n = String::new();
    println!("Please write the nth number: ");
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read the input");
    let n: u32 = n.trim().parse().expect("Bad input");

    let mut index: u32 = 0;
    let mut aux: u32;
    let mut values: (u32, u32) = (0, 1);
    while index < n {
        index += 1;
        aux = values.0;
        values.0 = values.1;
        values.1 = aux + values.1;
        println!("{}th value is: {}", index, values.0);
    }
}
