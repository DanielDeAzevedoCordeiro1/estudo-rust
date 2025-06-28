enum Mensagem {
    Mover { x: i32, y: i32 },
    Escrever(String),
    MudarCor(i32, i32, i32),
    Sair,
}

fn main() {
    let msg = Mensagem::Mover { x: 10, y: 20 };
    
    match msg {
        Mensagem::Mover { x, y } => {
            println!("Mover para posição ({}, {})", x, y);
        },
        Mensagem::Escrever(texto) => {
            println!("Texto: {}", texto);
        },
        Mensagem::MudarCor(r, g, b) => {
            println!("Mudar cor para RGB({}, {}, {})", r, g, b);
        },
        Mensagem::Sair => {
            println!("Saindo...");
            
        }
    }
}