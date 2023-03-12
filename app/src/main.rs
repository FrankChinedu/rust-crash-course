#![deny(clippy::all)]

fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{}, {}, ", r1, r2);
    let r3 = &mut s; // BIG PROBLEM
    println!("{}, ", r3);
}

/*
- Every value in rust has an owner
 - There can only be one owner at a time
 -  Once the owner goes out of scope the value is dropped

 #STRING
  is made up of three parts
   - a pointer to the memory that holds the contents of the string
   - a length
   - a capacity

   RULES of REFERENCES
   - At any given time, you can have either one mutable reference or any number of immutable refrences
   - References must alwyas be valid
*/
