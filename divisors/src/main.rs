fn find_divisors(integer: u32) -> Result<Vec<u32>, String> {
    if integer == 2 {
        return Err(format!("{} is prime", integer));
    }

    let mut divisors = Vec::new();

    for n in 2..=(integer / 2) {
        if integer % n == 0 {
            divisors.push(n);
        }
    }

    match divisors.len() {
        0 => Err(format!("{} is prime", integer)),
        _ => Ok(divisors),
    }
}

fn main() {
    assert_eq!(find_divisors(2), Err("2 is prime".to_string()));
    assert_eq!(find_divisors(3), Err("3 is prime".to_string()));
    assert_eq!(find_divisors(25), Ok(vec![5]));
    assert_eq!(find_divisors(100), Ok(vec![2, 4, 5, 10, 20, 25, 50]));
    assert_eq!(find_divisors(47), Err("47 is prime".to_string()));
    
    let integer = 72140u32;
    let divisors = find_divisors(integer);
    println!("found divisors:\n{:?}", divisors)
}
