struct Retangulo {
    largura: u32,
    altura: u32,
}

impl Retangulo {
    
    fn novo(largura: u32, altura: u32) -> Retangulo {
        Retangulo { largura, altura }
    }
    
   
    fn quadrado(tamanho: u32) -> Retangulo {
        Retangulo {
            largura: tamanho,
            altura: tamanho,
        }
    }
    
    fn area(&self) -> u32 {
        self.largura * self.altura
    }
}

fn main() {
    let ret = Retangulo::novo(10, 5);
    let quad = Retangulo::quadrado(7);
    
    println!("Área do retângulo: {}", ret.area());
    println!("Área do quadrado: {}", quad.area());
}