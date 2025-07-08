
#[derive(Debug, Clone, PartialEq)]
struct Ponto {
    x: f64,
    y: f64,
}

impl Ponto {
    fn new(x: f64, y: f64) -> Self {
        Ponto { x, y }
    }
    
    fn distancia(&self, outro: &Ponto) -> f64 {
        ((self.x - outro.x).powi(2) + (self.y - outro.y).powi(2)).sqrt()
    }
}

fn main() {
    let p1 = Ponto::new(0.0, 0.0);
    let p2 = Ponto::new(3.0, 4.0);

    println!("p1: {:?}", p1);
    
    let p3 = p1.clone();
    println!("p1 == p3: {}", p1 == p3);
    
    println!("Distância entre p1 e p2: {}", p1.distancia(&p2));
}