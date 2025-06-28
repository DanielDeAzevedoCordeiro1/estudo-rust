fn main() {
    let ponto = (3, 4);
    
    match ponto {
        (x, y) if x*x + y*y <= 25 => println!("Dentro do círculo com raio 5"),
        (x, y) => {
            let distancia = (x*x + y*y as f64).sqrt();
            println!("Fora do círculo, distância: {:.2}", distancia);
        }
    }
}