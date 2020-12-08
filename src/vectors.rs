// Vectors are resizeable Arrays 

use std::mem;

pub fn run(){    
    let mut numbers:Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
    //println!("{:?}",numbers );

    // Get single value
    println!("Ela is a cute girl and is {:?} years old", numbers[8] );

    // add to Vector
    numbers.push(6);
    numbers.pop();

    // get vector length 
    println!("length {}", numbers.len());

    // vectors are stack allocated
    println!("Vector occupies {} Bytes",mem::size_of_val(&numbers) );

    // Get slice
    let slice:&[i32] = &numbers[1..4];
    println!("slice {:?}", slice );

    // loop through vector values
    for x in numbers.iter(){
        println!("Number : {}", x);
    }

    // loop and mutate vector values
    for x in numbers.iter_mut(){
        *x *=2;
    }
    println!("Number vec : {:?}", numbers);
    

}