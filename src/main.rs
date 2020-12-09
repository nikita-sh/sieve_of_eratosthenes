use std::env;
use std::process::exit;

mod sieve;

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("[USAGE]: ./sieve_of_eratosthenes [n]");
        exit(1);
    }

    let n = match args[1].parse::<i32>() {
        Ok(n) => n,
        Err(_n) => {
            -1
        }
    };

    if n <= 2 {
        println!("There are no primes less than 2.");
        exit(1);
    }

    let primes : Vec<i32> = sieve::sieve_of_eratosthenes(&n);

    println!("Done. Primes less than {} are {:#?}", n, primes);
    exit(0);
}
