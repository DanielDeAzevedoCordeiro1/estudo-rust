fn main() {
    let string1 = String::from("olá");
    let string2 = String::from("mundo");
    
    let resultado = maior_string(&string1, &string2);
    println!("A maior string é: {}", resultado);
}

fn maior_string(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() { s1 } else { s2 }
}