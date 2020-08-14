extern crate rand;
use rand::Rng;
extern crate phrases;
use phrases::greetings::french; //to make it easier to call those functions
extern crate phrases2;
use phrases2::greetings2::french2; //to make it easier to call those functions

/*
lost of functionalities are available in the std library but many mor eare available as crates.
A crate is a package that you can compile on your machine. crates.io have many of them.
After checking what package you want, you edit Cargo.toml with the appropriate info.

Then, can use "cargo build"
//it will update the package registry
//download the required packages (the ones mentioned and their own dependencies)
//finally compile those packages

back to your code, you can now import with "extern crate rand;"
then "use rand::Rng;" for the specific module/function


 */

pub fn consuming_crates(){
    let mut rng = rand::thread_rng();
    let b =  rng.gen_bool(0.5);
    if b{ println!("yes b:{}",b); }
    else {  println!("no b:{}",b);}


}

pub fn creating_crate(){
    println!("English: {} and {}",
        phrases::greetings::english::hello(),
        phrases::greetings::english::goodbye());

    println!("French: {} and {}",
             french::hello(),
             french::goodbye());
}

pub fn creating_crate2(){
    println!("creating_crate2 using module inside the project directory");
    println!("English: {} and {}",
             phrases2::greetings2::english2::hello(),
             phrases2::greetings2::english2::goodbye());

    println!("French: {} and {}",
             french2::hello(),
             french2::goodbye());
}