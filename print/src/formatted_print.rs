pub fn formatted_print_examples() {
    println!("\nTesting arguments: {}", "this is the argument.");
    println!(
        "{2} {1} {0} because of positional arguments.",
        "This text", "is", "inverted"
    );
    // This won't work because positional arguments cannot come after named ones
    // println!("{argument} {1}", argument="hello", "hi")
    let formated_text = format!(
        "Testing format macro: {}",
        "this whole line was assigned to a variable."
    );
    println!("{}", formated_text);
    println!("This line uses {named_argument}", named_argument="named argument.");
    println!("Formatting the number 8 in binary: {:b}.", 8);
    println!("Pi is roughly {:.3}.\n", 3.14159265)
}
