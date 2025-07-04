
struct AlwaysEqual;

impl AlwaysEqual {
    fn equals(&self, _: &AlwaysEqual) -> bool {
        true
    }
}

fn main() {
    let a = AlwaysEqual;
    let b = AlwaysEqual;
    
    println!("a equals b? {}", a.equals(&b));
}