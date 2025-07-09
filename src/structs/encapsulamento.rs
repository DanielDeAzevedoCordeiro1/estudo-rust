mod geometria {
    pub struct Circulo {
        pub raio: f64,
        area: f64, 
    }
    
    impl Circulo {
        pub fn novo(raio: f64) -> Self {
            let area = std::f64::consts::PI * raio * raio;
            Circulo { raio, area }
        }
        
        pub fn get_area(&self) -> f64 {
            self.area
        }
        
        pub fn set_raio(&mut self, raio: f64) {
            self.raio = raio;
            self.area = std::f64::consts::PI * raio * raio; 
        }
    }
}

fn main() {
    use geometria::Circulo;
    
    let mut c = Circulo::novo(5.0);
    println!("Raio: {}", c.raio);
    println!("Área: {}", c.get_area());
    
    c.set_raio(10.0);
    println!("Nova área: {}", c.get_area());
    
}