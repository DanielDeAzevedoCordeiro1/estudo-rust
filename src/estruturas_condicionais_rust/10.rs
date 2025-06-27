fn main() {
    let referencia = &4;
    
    match referencia {
        &val => println!("Obtive um valor {} por meio da referência", val),
    }
    
    match *referencia {
        val => println!("Obtive um valor {} por meio da desreferenciação", val),
    }
}