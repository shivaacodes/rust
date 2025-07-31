#![allow(unused)]

fn main(){
    let x = -123;
    // x += 1  error , unmutable by default; This will not compile

    let mut y = -123; // mutable
    y += 2;
    println!("The value of y is {}", y);

    let z= -123;

    // shadowing
    let x = 1;
    let x= true;

    // vector
    let v: Vec<_>= vec![1,2,3];
}