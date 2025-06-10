fn main(){

    let a = 10;
    let b = 20;


    let c = a; 
    println!("c: {}", c);

    let mut d = b;
    d += 5;
    println!("d: {}", d);

    let e = take_ownership(c);
    println!("e: {}", e);
}

fn take_ownership(x: i32) -> i32 {
    println!("Taking ownership of: {}", x);
    x
}