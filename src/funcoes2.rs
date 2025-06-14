struct Calculadora {
    numero1: f64,
    numero2: f64,
}

impl Calculadora {
    fn new(numero1: f64, numero2: f64) -> Self {
        Calculadora { numero1, numero2 }
    }

    fn somar(&self) -> f64 {
        self.numero1 + self.numero2
    }

    fn subtrair(&self) -> f64 {
        self.numero1 - self.numero2
    }

    fn multiplicar(&self) -> f64 {
        self.numero1 * self.numero2
    }

    fn dividir(&self) -> f64 {
        if self.numero2 != 0.0 {
            self.numero1 / self.numero2
        } else {
            panic!("Divisão por zero não é permitida.");
        }
    }
}

use std::io;

fn main(){

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Digite o primeiro número:");
    io::stdin().read_line(&mut input1).expect("Erro ao ler o número");
    let numero1: f64 = input1.trim().parse().expect("Entrada inválida");

    println!("Digite o segundo número:");
    io::stdin().read_line(&mut input2).expect("Erro ao ler o número");
    let numero2: f64 = input2.trim().parse().expect("Entrada inválida");

    let calculadora = Calculadora::new(numero1, numero2);

    println!("Soma: {}", calculadora.somar());
    println!("Subtração: {}", calculadora.subtrair());
    println!("Multiplicação: {}", calculadora.multiplicar());
    println!("Divisão: {}", calculadora.dividir());
    println!("Operações concluídas com sucesso!");
}