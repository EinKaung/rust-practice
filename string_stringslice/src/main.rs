/// # String
/// String = owning a book.
/// # &str
/// &str = borrowing a few pages to read.
fn main() {
    // Ownership and borrowing
    let mut text = String::from("hello");
    text.push_str(" world");
    println!("text: {}", text); // hello world

    // String slices are immutable
    let slice = &text[0..7]; // last index is not included
    println!("slice:{}", slice); // hello

    // Ownership differences
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1: {}", s1); // error: use of moved value: `s1`
    println!("s2: {}", s2); // hello

    let slice3 = "HELLO";
    let slice4 = slice3;

    println!("slice3: {}", slice3); // HELLO
    println!("slice4: {}", slice4); // HELLO

    // Converting ownership
    let slice5 = "HELLO WORLD";
    let mut mutable_owned_string = slice5.to_string();
    mutable_owned_string.push_str("!!!");
    println!("mutable_owned_string: {}", mutable_owned_string); // HELLO WORLD!!!

    let mut mutable_owned_string2 = String::from("HELLO WORLD");
    mutable_owned_string2.push_str("***");
    println!("mutable_owned_string2: {}", mutable_owned_string2); // HELLO WORLD***

    // String → &str
    let string_data = String::from("Hello, everybody!");
    let string_slice: &str = &string_data;

    println!("string_slice: {}", string_slice); // Hello, everybody!
    println!("string_data: {}", string_data); // Hello, everybody!
    
}
