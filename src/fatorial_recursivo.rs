fn main(){
    println!("Digite um numero e descubra seu fatorial ...");

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let numero = buffer.trim().parse::<i32>().unwrap();
    let fatorial = calcular_fatorial(numero);

    println!("O fatorial de {} é {}", numero, fatorial);
}

fn calcular_fatorial(n: i32) -> i32 {
    match n {
        0 => 1,
        _ => n * calcular_fatorial(n - 1),
    }
}