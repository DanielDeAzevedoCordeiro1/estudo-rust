fn main() {
    let s1 = String::from("olá");
    
    let tamanho = calcular_tamanho(&s1);
    

    println!("'{}' tem {} bytes", s1, tamanho);
}


fn calcular_tamanho(s: &String) -> usize {
    s.len()
}