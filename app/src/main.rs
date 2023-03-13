#![deny(clippy::all)]

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

fn main() {
    let say_hello = |name: &str| name.to_string();
    // fn say_hello(name: &str) -> String {
    //     format!("{}", name)
    // }
    let greet = say_hello("f");
    println!("{}", greet);
}

//`{:?}` (or {:#?}
