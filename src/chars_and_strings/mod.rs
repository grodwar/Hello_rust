#![allow(dead_code)]

use rand;
use rand::Rng;
use std::io::stdin; // don't forget to add rand = "0.7.3" in the Cargo.toml dependencies!
//use std::io::stdin;

pub fn strings()
{
    // there are 2 string types
    //// type1 is a 'reference to a  static str'
    let s = "Hello there1";// let s:&'static str = ....
    // $str is a string slice , kinda a view into a string (like slices in an array)
    //println!(s); //format argument must be a string literal// its 'glued' at COMPILE time (statically allocated)
    //s = "pouet";  //Cannot assign twice to immutable variable [E0384],


    //FIDDLING around
    //doesn't work for its not a string literal... didn't figure it out yet
    //let c:char = s[0]; //the type `str` cannot be indexed by `{integer}`
    //let  b = &'b'.to_string();
    //println!("b is {}", b.to_string());
    //println!("c is {}",c.to_string());

    //yet we can get the characters as a sequence
    for c in s.chars().rev() // chars() gives a sequence of characters (called chars<_>)
    {
        println!("{}",c);
    }
    //dont forget the magic of "if let
    if let Some(first_char) = s.chars().nth(0)//trying to getting the first char
    {
        println!("first letter is {}", first_char)
    }

    //// type2 is a String (heap allocated and utf-8 valid sequence)
    // basically if you want a modifiable string, use String
    let mut letters = String::new();
    let mut a = 'a' as u8; // need to use 'as' to convert to u8
    while a <= ('z' as u8)
    {
        letters.push(a as char);
        letters.push_str(",");//note the 'push_str' and the '"' (double quote)
        a+=1;
    }
    println!("{}", letters);

    //conversion between String<>&str
    let _u:&str = &letters;//dref conversion magic

    //concatenation
    // String + str ==> just fine
    //let z = letters + "abc";
    //println!("z is now {}",z)
    //let z2 = letters + &letters; // should work?? but got "borrow of moved value: `letters`"

    //String from an str
    let abc = String::from("abc");
    println!("abc is >>{}<<", abc);
    //other option
    let mut bcd = "hello world".to_string();
    println!("bcd is >>{}<<", bcd);
    bcd.remove(0);
    bcd.push_str("!!!");
    println!("bcd is now >>{}<<", bcd.replace("ello", "goodbye"));


}

pub fn formatting_strings()
// all about the format macro!
{
    let name = "Edouard";
    let greeting = format!("Hi, I'm {}, nice to meet you!",name);
    println!("{}", greeting);

    let hello = "hello";
    let rust = "rust";
    let hello_rust = format!("{}, {}!", hello, rust);
    //println!(hello_rust);// doesn't work !
    println!("{}",hello_rust);

    let run = "run";
    let forest = "forest";
    let rfr = format!("{0}, {1} {0}!",run, forest);
    println!("{}",rfr);

    // referring to parameters using names
    let info = format!(
        "the name's {last}. {first} {last}",
        first="James",
        last="Bond"
    );
    println!("{}", info);

    let mixed = format!(
        "{1} {} {0} {} {data}",
        "alpha",
        "beta",
        //"gamma", //"error: argument never used"
        //gamma="gamma", //"error: named argument never used"
        data="delta"
    );
    println!("{}", mixed);
}


pub fn number_guessing_game(){
    let number:i64 = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Enter your guess:");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer)
        {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                //let parsed:i64 = buffer.trim().parse().unwrap();//another way google it
                println!("you entered {:?}",parsed);
                match parsed {
                    Ok(guess) =>
                        {
                            if guess <1 || guess > 100
                            {
                                println!("your guess is out of range")
                            }
                            else if guess < number
                            {
                                println!("your guess is too LOW")
                            }
                            else if guess > number
                            {
                                println!("your guess is too HIGH")
                            }
                            else
                            {
                                println!("your guess is RIGHT, bye!");
                                break;
                            }

                        },
                    Err(e) => println!("Could not read your input {} Try again!", e)
                }
            },
            Err(_) => continue //probably some bad input like some text

        }
    }

}