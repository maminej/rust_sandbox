/*
Primitive types 
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 ( number of bits vars take in memory )
Floats : f32, f64
Boolean: bool
Character : char
Tuples
Arrays
*/



pub fn run(){
    // default is i32
    let x = 2.5;

    // add explicit type
    let y:i64 = 23;
    println!("{} {}",x,y );

    // find max size
    println!("Max i32:{} / Max i64:{}", std::i32::MAX,std::i64::MAX);

    // get boolean from expression
    let is_greater: bool=10>5;
    println!("{}", is_greater);

    let a1='a';
    let face = '\u{1F600}';
    println!("{} {}",a1,face );

}