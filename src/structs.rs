
// traditional structs

struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct 
struct Color_t (u8, u8, u8);

// structs with functions 
struct Person {
    first_name: String,
    last_name: String
}
impl Person {
    // construct person 
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // get full name 
    fn full_name (&self)->String{
        format!("Person {} {} ", self.first_name, self.last_name)
    }

    // set last name
    fn set_last_name (&mut self, last: &str){
        self.last_name=last.to_string();
    }

    // Name to Tuple
    fn to_tuple(self) -> (String, String){
        (self.first_name ,self.last_name)
    }
}


pub fn run(){

let mut c = Color {
    red:255,
    green:0,
    blue:0
};

c.red = 200;

println!("Color : {},{},{} ", c.red,c.green,c.blue );

let c_t = Color_t(255,0,0);
println!("Color Tuple {},{},{}",c_t.0, c_t.1, c_t.2 );

let mut p = Person::new("John", "Doe");
println!("person {} {}",p.first_name, p.last_name );
p.set_last_name("Williams");
println!("{}",p.full_name() );
println!("Person Tuple : {:?}",p.to_tuple() );

}