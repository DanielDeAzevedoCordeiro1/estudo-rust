
struct Pessoa {
    nome: String,
    idade: u32,
}

fn main() {
    
    let pessoa1 = Pessoa {
        nome: String::from("Alice"),
        idade: 30,
    };
    

    println!("Nome: {}", pessoa1.nome);
    println!("Idade: {}", pessoa1.idade);
}