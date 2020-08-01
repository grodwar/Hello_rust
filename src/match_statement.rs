#![allow(dead_code)]


pub fn match_statement(){
    let country_code = 1000;

    let country = match country_code
    {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        33 => "France",
        1..=1000 => "Unknown", //range from 1 to 1000 inclusive (cause of "=")
        _ => "invalid" // "_" default case
    };

    println!("the country with the code {} is {}", country_code, country);


    let x = true;
    let s = match x
    {
        true => "yes",
        false => "no" // just to test the pattern check at compilation time
    };
    println!("s is {}", s);


}