fn main() {
    let temperatura = 25;
    let umidade = 70;
    
    match temperatura {
        t if t < 0 => println!("Congelando!"),
        t if t < 15 && umidade > 80 => println!("Frio e úmido"),
        t if t < 15 => println!("Frio"),
        t if t > 30 && umidade > 80 => println!("Muito quente e úmido"),
        t if t > 30 => println!("Muito quente"),
        _ => println!("Temperatura agradável"),
    }
}