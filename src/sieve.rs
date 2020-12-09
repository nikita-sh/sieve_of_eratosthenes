pub fn sieve_of_eratosthenes(n: &i32) -> Vec<i32> {
    let mut candidates : Vec<bool> = vec![true; *n as usize];
    candidates[0] = false;
    candidates[1] = false;
    
    for i in 2..((*n as f64).sqrt() as i32) {
        if candidates[i as usize] {
            let mut count = i32::pow(i, 2);
            let mut j = 1;

            while count < *n {
                candidates[count as usize] = false;
                count = i32::pow(i, 2) + i*j;
                j += 1; 
            }
        }
    }

    let mut primes : Vec<i32> = Vec::new();
    for (i, val) in candidates.iter().enumerate() {
        if *val {
            primes.push(i as i32);
        }
    }

    primes
}