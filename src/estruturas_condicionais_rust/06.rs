fn main() {
    let nota = 85;
    
    let conceito = match nota {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    };
    
    println!("Conceito: {}", conceito);
}