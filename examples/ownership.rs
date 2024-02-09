fn main() {
    println!("ownership");
    // let yo = String::from("yo");
    let yo: String = "yo".to_string();
    print_something(&yo);
    println!("main() {}",yo);

    let yo2 = String::from("yo2");
    println!("yo2 {}",yo2);
    let yo3 = add_to_string(yo2);
    println!("yo3 {}",yo3);

 
}


fn print_something(i: &String){
    println!("{i}")
}

fn add_to_string(mut p1: String) -> String {
    p1.push_str(" ohlala");
    p1
}