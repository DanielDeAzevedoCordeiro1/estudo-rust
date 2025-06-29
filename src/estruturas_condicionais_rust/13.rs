fn main() {
    let mut pilha = Vec::new();

    pilha.push(1);
    pilha.push(2);
    pilha.push(3);
    

    while let Some(valor) = pilha.pop() {
        println!("Valor desempilhado: {}", valor);
    }
}