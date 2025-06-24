fn main() {
    let codigo_status = 404;
    
    match codigo_status {
        200 => println!("OK"),
        404 => println!("Não encontrado"),
        418 => println!("Eu sou um bule de chá"),
        _ => println!("Outro código de status"), 
    }
}