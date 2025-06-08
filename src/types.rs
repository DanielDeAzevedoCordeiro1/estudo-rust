fn main() {

    let a : u8 = 255;
    let b: i16 = -32768;
    let c: u32 = 4294;
    let x: i32 = 5;
    let y: f64 = 3.14;
    let z: bool = true;
    let s: &str = "Hello, Rust!";
    let tuple: (u8, i16, u32, i32, f64, bool, &str) = (a, b, c, x, y, z, s);
    let string = String::from("Strings em rust sao armazenadas na heap");
    let array: [u8; 5] = [1, 2, 3, 4, 5];

    println!("Array: {:?}", array);
    println!("Tuple: {:?}", tuple);
    println!("String: {}", string); 
}
