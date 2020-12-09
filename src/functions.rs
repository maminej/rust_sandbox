// functions are used to store blocks of code for re-use

pub fn run() {
    greeting("Hello", "Jane");

    // bind function values to variables 
    let get_sum = add (5,5);
    println!("sum : {}", get_sum);

    // closure car use external variables
    let n3:i32 =10; 
    let add_nums = |n1:i32, n2:i32| n1 + n2 + n3;
    println!("Closure sum {}", add_nums(3,3));

}

fn greeting (greet: &str, name: &str)
{
    println!("{} {}, nice to meet you ", greet, name );
}

fn add (num1: i32, num2: i32)-> i32
{
    num1 + num2
}