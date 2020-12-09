pub fn run() {
let age=18;
let check_id:bool = false; 
let age_known:bool = true; 

// if statement 
if age>= 21 && check_id || age_known {
    println!("Bartender : what would you like to drink" );
}
else  if age<21 && check_id {
    println!("Bartender : sorry you have to leave" );
}
else {
    println!("Bartender : I need to see your ID" );
}


// short hand if 
let is_of_age = if age>=21 {true} else {false};
println!("{:?}",is_of_age );
}