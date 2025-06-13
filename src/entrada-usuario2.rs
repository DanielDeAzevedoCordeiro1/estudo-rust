use std::io;

fn main(){

    let nums = [1,2,3,10];
    println!("Digite um numero de 1 a 10 e descubra se esta no array ...");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");

    for (pos, num) in nums.iter().enumerate() {
        if num == &input.trim().parse::<i8>().unwrap() {
            println!("O numero {} esta no array na posicao {}", num, pos);
            return;
        }
    }

    println!("O numero digitado nao esta no array");
}