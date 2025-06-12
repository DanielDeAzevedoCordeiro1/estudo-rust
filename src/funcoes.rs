fn soma(a: i32, b: i32) -> i32 {
    a + b
}

fn imprime_soma(soma: i32) {
    println!("A soma é: {}", soma);
}


fn main(){

    let a = 10;
    let b = 20;

    let resultado = soma(a, b);

    imprime_soma(resultado);
}