fn main() {
  
    let mut s = String::from("Olá");
    
    let r1 = &s; 
    let r2 = &s; 
    println!("Referências imutáveis: {} e {}", r1, r2);

    let r3 = &mut s; 
    r3.push_str(", mundo!"); 
    println!("Após modificação: {}", r3);

    
 
    println!("String original (modificada): {}", s);
    
   
    imprimir(&s);
    adicionar_exclamacao(&mut s); 
    
    println!("String final: {}", s);
}


fn imprimir(texto: &String) {
    println!("Função imprimir: {}", texto);
}


fn adicionar_exclamacao(texto: &mut String) {
    texto.push_str("!!!");
}
