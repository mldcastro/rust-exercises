pub fn high_and_low(numbers: &str) -> String {
    let vector_of_ints: Vec<i32> = numbers
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    let highest = find_max(vector_of_ints.iter());
    let lowest = find_min(vector_of_ints.iter());

    return format!("{} {}", highest.unwrap(), lowest.unwrap()).to_string();
}

fn find_max<I>(iter: I) -> Option<I::Item>
where
    I: Iterator,
    I::Item: Ord,
{
    iter.reduce(|accum, val| if accum >= val { accum } else { val })
}

fn find_min<I>(iter: I) -> Option<I::Item>
where
    I: Iterator,
    I::Item: Ord,
{
    iter.reduce(|accum, val| if accum <= val { accum } else { val })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_output_is_correct_when_string_contains_only_positive_values() {
        assert_eq!(high_and_low("12 57 328 901 4738 10"), "4738 10");
    }

    #[test]
    fn assert_output_is_correct_when_string_contains_only_negative_values() {
        assert_eq!(high_and_low("-78 -90 -102 -2 -1 -23"), "-1 -102");
    }

    #[test]
    fn assert_output_is_correct_when_string_contains_negative_and_positive_values() {
        assert_eq!(high_and_low("8 3 -5 42 -1 0 -9 4 7 4 -4"), "42 -9");
    }

    #[test]
    fn assert_output_is_correct_when_string_contains_repeated_values() {
        assert_eq!(high_and_low("8 8 8 8 8 8 90 12 -7 -5 -5 -2 -7"), "90 -7");
    }
}

fn main() {
    let numbers: String = high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4");
    let result: Vec<&str> = numbers.split(" ").collect::<Vec<&str>>();

    println!(
        "Among the numbers (8 3 -5 42 -1 0 0 -9 4 7 4 -4) 
        the highest one is {highest} and the lowest is {lowest}.", 
        highest=result[0], lowest=result[1]
    );
}
