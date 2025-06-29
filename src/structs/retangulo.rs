struct Retangulo {
    largura: u32,
    altura: u32,
}

fn main() {
 
    let mut retangulo = Retangulo {
        largura: 10,
        altura: 5,
    };
    
    retangulo.largura = 20;
    
    println!("Dimensões: {} x {}", retangulo.largura, retangulo.altura);
    println!("Área: {}", retangulo.largura * retangulo.altura);
}