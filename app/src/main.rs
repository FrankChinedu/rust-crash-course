#![deny(clippy::all)]

const MY_AGE: u8 = 22;
fn main() {
    let name = "frank"; //string
    let name = 30u8; // integers
    let personal_data = (22, "dfd"); // tuples
    let frank = personal_data.0;
    // first_name = 50;
    println!("Hello {}!", name);
}
