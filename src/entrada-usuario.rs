use std::io;

fn main() {
    let mut buffer = String::new();

    println!("Digite algo ...");

    io::stdin()
        .read_line(&mut buffer)
        .expect("Houve um erro na leitura dos dados");

    println!("Dados digitados: {}", &buffer);
}
