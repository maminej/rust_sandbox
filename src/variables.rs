// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run()
{
    let name = "Amine";
    let mut age = 37;
    println!("my name is {} and I am {}", name, age);

    // define constant
    const ID: i32 =001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name,my_age) = ("Amine",37);
    println!("{} is {}",my_name, my_age );

}