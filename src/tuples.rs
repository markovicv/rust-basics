

//tuples are collection of values that are different types
pub fn tuple_fn(){

    //tuple
    let city : (&str,i32,&str) = ("Belgrade",2000000,"Serbia");

    println!("I live in {0} it is the capital of {2}, and {0} has {1} people living",city.0,city.1,city.2);

}