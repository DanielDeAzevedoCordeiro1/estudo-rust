// Exercicio 2 Verificador de Números Primos: Determine se um número é primo.

use std::io;

fn main(){

    println!("Digite um numero e descubra se ele é primo ...");

    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Houve um erro na leitura dos dados");

    let input = &buffer.trim().parse::<i32>().unwrap();

    if is_primo(input) {
        println!("O número {} é primo.", input);
    } else {
        println!("O número {} não é primo.", input);
    }
}

fn is_primo(num: &i32) -> bool{
    
    if *num == 0 {
       return false
    }
    
    match num{
        1 => false,
        2 => true,
        3 => true,
        _ => {
            for i in 2..=*num/2 {
                if num % i == 0 {
                    return false;
                }
            }
            true
        }
    }
}