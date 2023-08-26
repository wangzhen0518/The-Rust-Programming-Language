fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
fn main() {
    let mut s1 = String::from("hello");
    let word = first_word(&s1);
    let my_string_literal = "hello world";
    let word = first_word(my_string_literal);
    let x = &my_string_literal[..3];
    println!("{}", &my_string_literal[..3]);
}
