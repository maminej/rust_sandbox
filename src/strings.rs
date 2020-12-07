// Primitive str = immutable fixed-length string somewhere in memory 
// String = Growable, heap-allocated data srructure - Use when you need to modify or own string data

pub fn run(){
    let mut hello = String::from("Hello ");
    // get Length 
    println!("Legth :{}",hello.len() );
    
    // String push char and a string
    hello.push('W');
    hello.push_str("orld!");
    println!("{}",hello );

    // Get capacity in Bytes
    println!("capacity : {}", hello.capacity());

    //check if empty 
    println!("is empty ? :{}",hello.is_empty() );

    // contains and replace string 
    println!("contains 'world' :{}",hello.contains("World") );
    println!("replace  :{}",hello.replace("World","There") );

    // loop through string by whitespaces

    for word in hello.split_whitespace() {
        println!("{}",word );
    }

}
