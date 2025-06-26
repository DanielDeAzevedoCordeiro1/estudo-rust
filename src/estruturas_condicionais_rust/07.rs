fn main() {
    let algum_valor: Option<i32> = Some(42);
    
    if let Some(valor) = algum_valor {
        println!("Encontrou um valor: {}", valor);
    } else {
        println!("Nenhum valor encontrado");
    }
}