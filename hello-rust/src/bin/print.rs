#![allow(unused)]

#[derive(Debug)] // tells rust to autogenerate a code
struct Lang {
    language: String,
    version: String,
}
fn main() {
    let lang = "rust";
    let bang = "hola!";
    println!("hello {}", lang);
    println!("hello {} {}", lang, bang);
    println!("hello {bang}");

    let x = 2;
    println!("{0} x {0} = {1}", x, x * x);

    let lang = Lang {
        language: "rust".to_string(),
        version: "1.88".to_string(),
    };

    println!("{:?}", lang); // prints using struct
    println!("{:#?}", lang); // line breaks
}
