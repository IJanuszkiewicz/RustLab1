use std::fs::File;
use rand::Rng;
use std::io::prelude::*;

const MAX_COLLATZ_ITERATIONS: u64 = 100;
const TAB_SIZE: usize = 10;

fn main() {
    let exit_error = loop{
        println!("Podaj liczbę");
        let mut number = String::new();
        std::io::stdin().read_line(&mut number).expect("Error reading line");

        let mut number: u64 = match number.trim().parse(){
            Ok(num) => num,
            Err(_) => break true,
        };

        println!("Twoja liczba to {}", number);

        if number == 0 {
            break false;
        }

        let random_number = rand::rng().random_range(0..=5);
        number += random_number;
        println!("Twoja liczba po dodaniu to {}", number);

        let powers = get_powers(number);
        let collatz_results = check_collatz_tab(powers);

        let mut file = match File::create("xyz.txt"){
            Ok(file) => file,
            Err(_) => break true,
        };

        let mut file_string = String::new();
        file_string.push_str(number.to_string().as_str());
        file_string.push('\n');
        for power in powers{
            file_string.push_str(&format!("{} ", power));
        };
        file_string.push('\n');
        for result in collatz_results{
            file_string.push_str(&format!("{} ", result));
        }
        file_string.push('\n');

        match file.write_all(file_string.as_bytes()){
            Ok(_) => (),
            Err(_) => break true,
        }

    };

    match exit_error {
        true => {println!("Pętla zakończona przez błąd")}
        false => {println!("Pętla zakończona przez użytkownika")}
    }
    let numbers = [1,2,3,4,5,6,7,8,9,10];
    let (has_composite, composite) = find_composite(numbers);
    if has_composite {
        println!("Tablica ma liczbe złożoną: {}", composite);
    }
    else {
        println!("Tablica nie ma liczby złożonej");
    }
}

fn get_powers(x: u64) -> [u64; TAB_SIZE] {
    let mut tab = [1u64;TAB_SIZE];
    for exponent in 1usize..TAB_SIZE {
       tab[exponent] = x*tab[exponent - 1];
    }
    tab
}

fn collatz_function(n: u64) -> u64{
    if n.is_power_of_two(){
         n/2
    } else {
        3*n + 1
    }
}

fn check_collatz(mut n: u64) -> bool{
    for _ in 0..MAX_COLLATZ_ITERATIONS{
        if n == 1 {
            return true;
        }
        n = collatz_function(n);
    }
    false
}

fn check_collatz_tab(numbers: [u64; TAB_SIZE]) -> [bool; TAB_SIZE] {
    let mut result = [false; TAB_SIZE];
    for i in 0usize..TAB_SIZE{
        result[i] = check_collatz(numbers[i]);
    }
    result
}

// returns true and composite when numbers contain a composite, false and 0 otherwise
fn find_composite(numbers: [u64; TAB_SIZE]) -> (bool, u64) {
    let mut result = (false , 0u64);
    'outer: for number in numbers {
        let mut i = 2;
         while i * i <= number {
            if number % i == 0 {
                result.0 = true;
                result.1 = number;
                break 'outer;
            }
            i += 1;
        }
    };
    result
}