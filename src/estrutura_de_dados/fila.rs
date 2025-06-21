struct Fila<T> {
    itens: Vec<T>,
}

impl<T> Fila<T> {

    fn nova() -> Self {
        Fila { itens: Vec::new() }
    }
    
    fn adicionar(&mut self, item: T) {
        self.itens.push(item);
    }
   
    fn remover(&mut self) -> Option<T> {
        if self.itens.is_empty() {
            None
        } else {
            Some(self.itens.remove(0))
        }
    }
    
    fn vazia(&self) -> bool {
        self.itens.is_empty()
    }
}

fn main() {
    let mut fila = Fila::nova();
    
    fila.adicionar(1);
    fila.adicionar(2);
    fila.adicionar(3);
    
    println!("Removendo itens da fila:");
    while let Some(item) = fila.remover() {
        println!("{}", item);
    }
    
    println!("Fila vazia: {}", fila.vazia());
}