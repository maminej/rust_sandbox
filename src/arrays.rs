// Arrays are fixed lists where elements are the same data types

use std::mem;

pub fn run(){    let numbers:[i32;10] = [1,2,3,4,5,6,7,8,9,10];
    //println!("{:?}",numbers );

    // Get single value
    println!("Ela is a cute girl and is {:?} years old", numbers[8] );

    // get array length 
    println!("length {}", numbers.len());

    // arrays are stack allocated
    println!("Array occupies {} Bytes",mem::size_of_val(&numbers) );

    // Get slice
    let slice:&[i32] = &numbers[1..4];
    println!("slice {:?}", slice );

}