// tuples group together values of different types
// MAX of 12 elements

pub fn run()
{
    let person:(&str,&str,i8)=("Amine","Tunisia",37);
    println!("{} is from {} and is {}",person.0,person.1,person.2 );

}