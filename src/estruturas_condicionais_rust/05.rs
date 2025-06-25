fn main() {
    let codigo_status = 404;
    
    let mensagem = match codigo_status {
        200 => "OK",
        404 => "Não encontrado",
        418 => "Eu sou um bule de chá",
        _ => "Código desconhecido",
    };
    
    println!("Status: {}", mensagem);
}