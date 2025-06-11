fn main(){

    let l0 = "teste";
    println!("l0: {}", l0);

    let l1 = &l0;
    println!("l1: {}", l1);

    let mut string1 = String::new();
    string1.push_str("Hello, ");
    string1.push_str("world!");
    println!("string1: {}", string1);

    let string2 = String::from("Hello, Rust!");
    println!("string2: {}", string2);

}