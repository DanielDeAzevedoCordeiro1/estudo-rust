use std::io;

fn main(){

    let mut buffer = String::new();
    println!("Digite um numero e descubra seu fatorial ...");

    io::stdin().read_line(&mut buffer).unwrap();
    let numero = buffer.trim().parse::<i32>().unwrap();

    let fatorial = calcular_fatorial(numero);
    println!("O fatorial de {} é {}", numero, fatorial);
}

fn calcular_fatorial(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        n * calcular_fatorial(n - 1)
    }
}