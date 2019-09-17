
pub fn immutable_vars(){
    let name = "Igor";
    let age = 23;
    
    println!("My name is {} I am {} years old",name,age);

}

pub fn mutable_vars(){
    let name  = "Igor";
    let mut age = 23;

    println!("My name is {} I am {} years old",name,age);

    age = 24;
    println!("Next year i will be {} years old",age);

}
pub fn explicit_vars(){

    //32 bit integer
    let x : i32 = 69472;
    let x2 : i32 = 443;
    //64 bit integer
    let y : i64 = 123123123;
    //64 bit floating number
    let z : f64 = 43.54;
    //characters
    let c : char = 'A';
    //boolean valuse
    let running : bool = true;
    // expression booleans
    let is_bigger: bool = x>x2;

    println!("{:?}",(x,y,z,c,running,is_bigger));




}