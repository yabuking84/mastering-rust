fn main() {
    println!("BORROWING");

    // let mut s1 = String::from("hello");
    // let r1 = &s1;
    // print_something(r1);
    // let r2 = &mut s1;
    // r2.push_str("xxxxxxxx");
    // add_to_string(r2);

    // println!("s1 is {s1}");


    // let mut str1 = String::from("modifiable");
    // let str2 = String::from("fixed string");
    // let mut str_ptr: &String;
    // str_ptr = &str1;
    // println!("ptr currently points to {str_ptr}");
    // str_ptr = &str2;
    // println!("ptr currently points to {str_ptr}");
    // str1.push_str(" string");
    // str_ptr = &mut str1;
    // println!("ptr currently points to {str_ptr}");


    // let s1 = String::from("hello");
    // let r1 = &s1;
    // let s1 = add_to_string_own(s1);

    // println!("s1 is {s1}");

    let str1 = String::from("modifiable");
    let str2 = String::from("fixed string");
    let mut str_ptr: &String;
    str_ptr = &str1;
    println!("ptr currently points to {str_ptr}");
    str_ptr = &str2;
    println!("ptr currently points to {str_ptr}");
    // str1.push_str(" string");
    str_ptr = &str1;
    println!("ptr currently points to {str_ptr}");

    

}

fn add_to_string_own(mut p1: String) -> String  {
    // (*p1).push_str(" pushed"); // not needed, Rust has automatic dereference 
    p1.push_str(" pushed");
    p1
}

fn add_to_string(p1: &mut String)  {
    // (*p1).push_str(" pushed"); // not needed, Rust has automatic dereference 
    p1.push_str(" pushed");
}

fn print_something(i: &String){
    println!("print_something() {i}");
}