
struct Cor(u8, u8, u8);

impl Cor {
    fn rgb(&self) -> String {
        format!("rgb({}, {}, {})", self.0, self.1, self.2)
    }
    
    fn vermelho() -> Cor {
        Cor(255, 0, 0)
    }
    
    fn verde() -> Cor {
        Cor(0, 255, 0)
    }
    
    fn azul() -> Cor {
        Cor(0, 0, 255)
    }
}

fn main() {
    let preto = Cor(0, 0, 0);
    let vermelho = Cor::vermelho();
    
    println!("Preto em RGB: {}", preto.rgb());
    println!("Vermelho em RGB: {}", vermelho.rgb());
}