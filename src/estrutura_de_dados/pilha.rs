struct Pilha<T> {
    itens: Vec<T>,
}

impl<T> Pilha<T> {

    fn nova() -> Self {
        Pilha { itens: Vec::new() }
    }
    

    fn empilhar(&mut self, item: T) {
        self.itens.push(item);
    }
    
 
    fn desempilhar(&mut self) -> Option<T> {
        self.itens.pop()
    }
    

    fn vazia(&self) -> bool {
        self.itens.is_empty()
    }
    

    fn tamanho(&self) -> usize {
        self.itens.len()
    }
}

fn main() {
    let mut pilha = Pilha::nova();
    
                
    pilha.empilhar(1);
    pilha.empilhar(2);
    pilha.empilhar(3);
    
    println!("Tamanho da pilha: {}", pilha.tamanho());
    
             
    println!("Removendo itens da pilha:");
    while let Some(item) = pilha.desempilhar() {
        println!("{}", item);
    }
    
    println!("Pilha vazia: {}", pilha.vazia());
}