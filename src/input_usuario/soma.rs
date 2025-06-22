use std::io;

fn main() {

    let mut buffer = String::new();
    println!("Digite um numero: ");
    
    io::stdin().read_line(&mut buffer).expect("Deu ruim");
    
    let num = buffer.trim().parse::<i32>().unwrap();
    println!("Soma = {}",soma(&num));
}

fn soma(num: &i32) -> i32{
    match num {
        0 => 0,
        1 => 1,
        _ => num + soma(&(num - 1))
    }
}
