struct Retangulo {
    largura: u32,
    altura: u32,
}

fn calcular_area(retangulo: &Retangulo) -> u32 {
    retangulo.largura * retangulo.altura
}

fn main() {
    let retangulo = Retangulo {
        largura: 10,
        altura: 5,
    };
    
    let area = calcular_area(&retangulo);
    println!("Área do retângulo: {}", area);
}