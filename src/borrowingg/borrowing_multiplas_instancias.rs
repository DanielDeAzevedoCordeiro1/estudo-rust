fn main() {
    let mut s = String::from("olá");
    
    let r1 = &s;
    let r2 = &s;
    println!("{} e {}", r1, r2);
    
    let r3 = &mut s;
    r3.push_str(" mundo");
    println!("{}", r3);
    
}