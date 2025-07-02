
struct Par<T> {
    x: T,
    y: T,
}

impl<T> Par<T> {
    fn novo(x: T, y: T) -> Self {
        Par { x, y }
    }
}

impl Par<f64> {
    fn distancia_da_origem(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let inteiros = Par::novo(5, 10);
    let pontos = Par::novo(3.0, 4.0);
    
    println!("Par de inteiros: ({}, {})", inteiros.x, inteiros.y);
    println!("Distância da origem: {}", pontos.distancia_da_origem());
}