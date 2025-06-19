fn main() {
    let s = String::from("olá mundo");
    
    tomar_posse(s);
    
    let x = 5;
    fazer_copia(x);
    println!("{}", x);
}

fn tomar_posse(string: String) {
    println!("{}", string);
}

fn fazer_copia(inteiro: i32) {
    println!("{}", inteiro);
} 