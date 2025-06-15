//Exercicio 1 Fibonacci: Calcule os primeiros N números da sequência de Fibonacci.


use std::io;

fn main(){
    println!("Digite a posicao na sequencia de fibonnaci e descubra o valor ...");

    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Houve um erro na leitura dos dados");

    let input = &buffer.trim().parse::<i16>().unwrap();

    println!("{}",&input);

    println!("Valor do indice => {} na sequencia de fibonacci e {}",&input,fibonacci(&input))

    
}

fn fibonacci(indice: &i16) -> i16{
    match indice {
        0 => 0,
        1 => 1,
        _ => fibonacci(&(indice - 2)) + fibonacci(&(indice -1))
    }
}