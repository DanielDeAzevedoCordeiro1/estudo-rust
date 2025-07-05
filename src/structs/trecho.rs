
struct Trecho<'a> {
    texto: &'a str,
}

impl<'a> Trecho<'a> {
    fn novo(texto: &'a str) -> Self {
        Trecho { texto }
    }
    
    fn primeira_palavra(&self) -> &'a str {
        match self.texto.find(' ') {
            Some(pos) => &self.texto[..pos],
            None => self.texto,
        }
    }
}

fn main() {
    let texto = String::from("Olá mundo, como vai?");
    let trecho = Trecho::novo(&texto);
    
    println!("Texto completo: {}", trecho.texto);
    println!("Primeira palavra: {}", trecho.primeira_palavra());
}