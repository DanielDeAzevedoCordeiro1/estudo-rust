fn main(){
    let mut vetor: Vec<i32> = Vec::new();
    vetor.push(10);
    vetor.push(20);
    vetor.push(30);

    println!("Vetor: {:?}", vetor);

    if let Some(valor) = vetor.get(1) {
        println!("Elemento no índice 1: {}", valor);
    } else {
        println!("Índice fora dos limites do vetor");
    }

    for valor in &vetor {
        println!("Valor: {}", valor);
    }

    if let Some(valor_removido) = vetor.pop() {
        println!("Valor removido: {}", valor_removido);
    } else {
        println!("Vetor vazio, nada para remover");
    }

    println!("Vetor após remoção: {:?}", vetor);
}