fn main() {
    let string1 = String::from("olá longo");
    let string2 = String::from("mundo");
    
    let resultado = maior_string(&string1, &string2);
    println!("A maior string é: {}", resultado);
}

fn maior_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}