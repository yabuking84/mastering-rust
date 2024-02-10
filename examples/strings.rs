fn main() {
    println!("STRINGS");
    let mut a = "xxx".to_string();
    let mut d1: &String = &a;
    let d2: &mut String = &mut a;

    let _s1 = "😂 옻옺웃ӱ";
    let _s2 = String::from("😂 옻옺웃");
    let _s3 = "😂 옻옺웃".to_string();
    let s4 = "😂 옻옺웃".to_owned();
    let s5 = &s4[..];

    println!("{}", s5);


    let mut sa1 = "Hello".to_string();
    let sa2 = " world".to_string();

    sa1 = sa1 + &sa2;
    println!("sa3:{}", sa1);

    let ssss = String::from("hello world");

    let world = &ssss[6..11];

}
