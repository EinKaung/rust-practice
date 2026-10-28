fn main() {
    println!("Hello, world!");

    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("my_array: {:?}", my_array);

    let my_tuple: (i32, f64, char) = (1, 2.0, '3');
    println!("my_tuple: {:?}", my_tuple);

    let mut mutable = 12;
    mutable = 15;
    let mutable = 21.11;
    println!("mutable: {mutable}");

    // all 5 elements in the array are initialized to 77
    let equal_array: [i32; 5] = [70; 5];
    println!("equal_array: {:?}", equal_array);
}
