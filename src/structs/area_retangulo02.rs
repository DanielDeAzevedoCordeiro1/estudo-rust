struct Retangulo {
    largura: u32,
    altura: u32,
}

impl Retangulo {
 
    fn area(&self) -> u32 {
        self.largura * self.altura
    }
    

    fn contem(&self, outro: &Retangulo) -> bool {
        self.largura > outro.largura && self.altura > outro.altura
    }
}

fn main() {
    let ret1 = Retangulo {
        largura: 10,
        altura: 5,
    };
    
    let ret2 = Retangulo {
        largura: 8,
        altura: 3,
    };
    
    println!("Área do retângulo 1: {}", ret1.area());
    println!("Retângulo 1 contém retângulo 2? {}", ret1.contem(&ret2));
}