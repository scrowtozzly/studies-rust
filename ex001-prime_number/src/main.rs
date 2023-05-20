use std::{println, io};

fn main() {
    let mut start_number = String::new();
    let mut end_number = String::new(); 
    
    println!("Por favor, entre dois numeros para criarmos um intervalo de numeros e acharmos todos os numeros primos nesse intervalo. ");

    io::stdin()
        .read_line(&mut start_number)
        .expect("Por favor entre um numero inteiro.");

    io::stdin()
        .read_line(&mut end_number)
        .expect("Por favor entre um numero inteiro");
    
    let start_number: i32 = start_number.trim().parse().expect("Nao e um inteiro");
    let end_number: i32 = end_number.trim().parse().expect("Nao e um inteiro");

    for number_prime in start_number..end_number {
        if is_prime_number(number_prime) {
            println!("Achei um numero primo: {}", number_prime);
        }
    }
}

fn is_prime_number(number: i32) -> bool {   
    if number < 2 { 
        return false;
    }
    
    for counter in 2..(number / 2 + 1) {
        if number % counter == 0 {
            return false;
        }
    }
    true
}
