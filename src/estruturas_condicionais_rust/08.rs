fn main() {
    let ponto = (10, 20);
    
    match ponto {
        (0, 0) => println!("Origem"),
        (0, y) => println!("Eixo Y em {}", y),
        (x, 0) => println!("Eixo X em {}", x),
        (x, y) => println!("Ponto em ({}, {})", x, y),
    }
}