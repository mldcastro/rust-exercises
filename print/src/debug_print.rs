#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

pub fn debug_examples() {
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );
    println!("Printing Strucuture: {:?}", Structure(3));
    println!("Printing Deep: {:?}", Deep(Structure(3)));
    
    let name = "Peter";
    let age = 27;
    let peter = Person {name, age};

    println!("Pretty printing: {:#?}", peter);
}
