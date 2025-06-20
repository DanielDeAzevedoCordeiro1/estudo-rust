fn main() {
    let r;
    {
        let x = 5;
        r = &x; // Isso não compila!
    } // x sai de escopo aqui
    
    // println!("r: {}", r); // Tentaria usar uma referência inválida
}