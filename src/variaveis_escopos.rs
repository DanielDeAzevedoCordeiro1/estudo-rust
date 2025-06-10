fn main(){

    let a = {
        let b = 10;
        b + 5
    };

    println!("The value of a is: {:?}", a);

    let a = 23;

    println!("The value of a is: {:?}", a);

    {
        let a = 42;
        println!("The value of a inside the block is: {:?}", a);
    };

    println!("The value of a outside the block is: {:?}", a);
}