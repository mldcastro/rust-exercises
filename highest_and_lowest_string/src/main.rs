fn high_and_low_v1(numbers: &str) -> String {
    let vector_of_ints: Vec<i32> = numbers
          .split(" ")
          .collect::<Vec<&str>>()
          .iter()
          .map(|num| num.parse::<i32>().unwrap())
          .collect();
  
    let highest = find_max(vector_of_ints.iter());
    let lowest = find_min(vector_of_ints.iter());

    return format!("{} {}", highest.unwrap(), lowest.unwrap());
}

fn find_max<I>(iter: I) -> Option<I::Item> 
        where I: Iterator, I::Item: Ord,
        {
        iter.reduce(|accum, val| {
            if accum >= val {accum} else {val}
        })
    }

fn find_min<I>(iter: I) -> Option<I::Item> 
    where I: Iterator, I::Item: Ord,
    {
    iter.reduce(|accum, val| {
        if accum <= val {accum} else {val}
    })
}

fn main() {
    high_and_low_v1("8 3 -5 42 -1 0 0 -9 4 7 4 -4");
}
  