fn main() {
    let s = String::from("Los amigos son amigos para siempre y por siempre");
    let index = first_word(&s);
    println!("the index of the first word is: {}", index);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // For now, know that iter is a method that returns each element in a
    // collection and that enumerate wraps the result of iter and returns each
    // element as part of a tuple instead. This is a bit more convenient than
    // calculating the index ourselves
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}