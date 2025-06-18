fn main() {
    let mut s = String::from("olá");
    adicionar_mundo(&mut s);
    
    println!("{}", s); 
}

fn adicionar_mundo(s: &mut String) {
    s.push_str(" mundo");
}