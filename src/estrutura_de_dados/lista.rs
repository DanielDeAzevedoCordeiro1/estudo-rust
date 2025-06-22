struct Lista<T> {
    elementos: Vec<T>,
}

impl<T> Lista<T> {

    fn nova() -> Self {
        Lista {
            elementos: Vec::new(),
        }
    }
    
    fn adicionar(&mut self, item: T) {
        self.elementos.push(item);
    }
 
    fn remover(&mut self, indice: usize) -> Option<T> {
        if indice < self.elementos.len() {
            Some(self.elementos.remove(indice))
        } else {
            None
        }
    }

    fn obter(&self, indice: usize) -> Option<&T> {
        self.elementos.get(indice)
    }

    fn tamanho(&self) -> usize {
        self.elementos.len()
    }
    

    fn vazia(&self) -> bool {
        self.elementos.is_empty()
    }
}

fn main() {
    let mut lista = Lista::nova();
    
 
    lista.adicionar(10);
    lista.adicionar(20);
    lista.adicionar(30);
    

    println!("Lista de tamanho: {}", lista.tamanho());
    
    for i in 0..lista.tamanho() {
        if let Some(valor) = lista.obter(i) {
            println!("Elemento {}: {}", i, valor);
        }
    }

    if let Some(removido) = lista.remover(1) {
        println!("Elemento removido: {}", removido);
    }
    
 
    println!("\nLista após remoção:");
    for i in 0..lista.tamanho() {
        if let Some(valor) = lista.obter(i) {
            println!("Elemento {}: {}", i, valor);
        }
    }
    
    println!("Lista vazia? {}", lista.vazia());
}