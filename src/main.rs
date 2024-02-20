use num::integer::sqrt;

use hello::greet;
use rand::Rng;
// use std::collections::HashMap; // standart library always available by default
const LANGUAGE2: u8 = 1;
static LANGUAGE: &str = "Rust";

fn main() {
    // let bunnies = 0.5;

    // println!("Hello, world! 1");
    // println!("Hello, world! 2");
    // println!("Hello, world! 3");
    // println!("Hello, world! 4 x {}", bunnies);
    // {

    //     let bunnies = 3;
    //     println!("Hello, world! 5 x {}", bunnies);
    // }
    // println!("Hello, world! 6 x {}", bunnies);

    // greet();

    let x = rand::thread_rng().gen_range(0, 100);

    println!("Hello, world!  {}", x);

    // let num = 4;

    // let msg = if num == 5 {
    //     "five"
    // } else if num == 4 {
    //     "four"
    // } else {
    //     "other"
    // };

    // 'bob: loop {
    //     loop {
    //         loop {
    //             break 'bob;
    //         }
    //     }
    // }

    // println!("Hello, world!  {}", msg);

    // for num in [7, 8, 9].iter() {
    //     println!("num = {}", num)
    // }

    // let hey = "abcd";
    // let hey_thai = "สวัสดี";

    // // for c in hey.bytes() {
    // //     println!("{}", c);
    // // }

    // println!("{:?}", hey.bytes().nth(2));
    // println!("{}", hey.chars().nth(2).unwrap());
    // println!("{}", hey.split("c").next().unwrap());

    // for c in hey_thai.bytes() {
    //     print!("{} ", c);
    // }
    // println!();

    // let mut s1 = String::from("hello");
    // let s2 = s1;
    // s1 = "cccc".to_string();
    // println!("{} ", s1);
    // println!("{} ", s2);

    // do_stuff(&s1);
    // println!("{} ", s1);

    // struct Data {
    //     isGo: bool,
    // }

    // trait Human {
    //     fn new() -> Data;
    //     fn walk();
    // }

    // let some = "x";

    // impl Human for Data {
    //     fn new() -> Data {
    //         Data { isGo: false }
    //     }

    //     fn walk(){
    //         println!("walking");
    //     }
    // }

    // let tawing: Data = Human::new();

    // let my_first_initial = 'C';

    // if my_first_initial.is_alphabetic() {
    //     println!("Alphabetical!");
    // } else if my_first_initial.is_numeric() {
    //     println!("Numerical!");
    // } else {
    //     println!("Neither alphabetic nor numeric!");
    // }

    // let // Finish this line like the example! What's your favorite character?
    // Try a letter, try a number, try a special character, try a character
    // from a different language than your own, try an emoji!

    // if your_character.is_alphabetic() {
    //     println!("Alphabetical!");
    // } else if your_character.is_numeric() {
    //     println!("Numerical!");
    // } else {
    //     println!("Neither alphabetic nor numeric!");
    // }

    let numbers = [36, 25, 49, 47, 3, 64, 16, 9];
    let prime = get_prime(numbers);

    // println!("{}", sqrt(numbers[3]).to_string());
}

fn get_prime(arr: [i32; 8]) -> i32 {
    // Loop through every element in the array
    let mut ii = 0;
    let mut is_prime = false;
    'outer: loop {
        is_prime = false;
        // Find a divisor.
        let mut n = 2;
        'inner: loop {
            // If a number can be evenly divided by another number except 1 and itself,
            // then it's not a prime.
            // Break out here to move on to the next element.
            if arr[ii] % n == 0 {
                if arr[ii] == 2 {
                    break 'outer;
                }
                ii += 1;
                break 'inner;
            }

            // If no divisors are found, then we've found a prime!
            // Break out of the loop here.
            // if n >= sqrt(arr[ii]) {
            println!("Inner Prime number {} {}", n, arr[ii] - 1);
            if n == arr[ii] - 1 {
                println!("Inner 2 Prime number {} {}", n, arr[ii] - 1);
                // break 'outer;
                is_prime = true;
                break 'inner;
            }

            // Otherwise, move to the next element.
            n += 1;
        }

        if is_prime {
            println!("Prime number {} {}", arr[ii], is_prime);
            break 'outer;
        }
    }
    println!("The first prime number in the array is {}.", arr[ii]);
    arr[ii]
}

// fn do_stuff(s: &String){
//     // s.insert_str(0, " xxxx");
//     println!("do_stuff() {}", s);
//     let mut  c = *s;
//     c = "yoyoyoyo".to_string();
//     println!("do_stuff() {}", c);

// }
