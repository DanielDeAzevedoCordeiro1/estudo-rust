fn main() {
    let numero = 7;
    
    let mensagem = if numero % 2 == 0 {
        "O número é par"
    } else {
        "O número é ímpar"
    };
    
    println!("{}", mensagem);
}