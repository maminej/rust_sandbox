pub fn run(){
    // print to console 
    println!("print from print.rs file");

    // Basic formatting
    println!("{0} is an {2} {1}",1,"Number","Integer");

    // Named parameters 
    println!("{number} is a {type}",
    number= 1.1,
    type="float" );

    // placeholder traits
    println!("Binary: {:b} Hex: {:x} octal: {:o}",0xaa,0xaa,0xaa );

    // placeholder for debug trait : Tupil in this case
    println!("{:?}",(12,true,"hello") );

}