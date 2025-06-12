fn main(){

    let numbers: [i8; 19] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19];
    let mut sum: i8 = 0;
    let mut count: i8 = 0;

    for number in numbers.iter() {
        if *number % 2 == 0 {
            sum += number;
            count += 1;
        }
    }

    println!("A soma dos números pares é: {}", sum);
    println!("A quantidade de números pares é: {}", count);
    
}