use std::{println, io};

fn main() {
    let mut number_input = String::new();

    println!("Por favor, de entrada em um numero inteiro positivo para eu verificar se ele e um numero perfeito. ");

    io::stdin()
        .read_line(&mut number_input)
        .expect("Inclua por favor um numero inteiro.");

    let number_input: u32 = number_input.trim().parse().expect("Um numero inteiro por favor"); 
   
    if number_input < 0 {
        println!("Deve ser maior que zero");
        return;
    }

    if is_perfect_number(number_input) {
        println!("E um numero perfeito.");
    } else {
        println!("Nao e um numero perfeito");
    }
}

fn is_perfect_number(number: u32) -> bool {
    let mut sum = 0;

    for divisors in 1..number {
        if number % divisors == 0 {
            sum += divisors;
        }
    }

    number == sum
}
