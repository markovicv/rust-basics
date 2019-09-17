

/*
Two types of strings:
    1. String - heap allocated,growable,not null terminated
    2. Primitive str - Immutable fixed-length
*/
pub fn string_manipulation(){

    let mut my_string = String :: from("My name is ");

    //pushes a char to my_string
    my_string.push('I');
    //pushes a string to my_string
    my_string.push_str("gor");

    println!("{}",my_string);

    //contains
    println!("my_string contains 'name' {}",my_string.contains("name"));

}