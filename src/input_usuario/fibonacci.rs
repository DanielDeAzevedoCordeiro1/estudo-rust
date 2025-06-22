use std::io;

fn main() {

    let mut buffer = String::new();
    println!("Digite um numero: ");
    
    io::stdin().read_line(&mut buffer).expect("Deu ruim");
    
    let num = buffer.trim().parse::<i32>().unwrap();
    println!("Fibonacci = {}",fibonacci(&num));
}

fn fibonacci(num: &i32) -> i32{
    match num{
        0 => 0,
        1 => 0,
        2 => 1,
        _ => fibonacci(&(num - 1)) + fibonacci(&(num - 2))
        
    }
}