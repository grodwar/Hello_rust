#![allow(dead_code)] //turns off the warning from the compiler
//#![allow(unused_imports)]
use std::mem; // this is how you import

const MEANING_OF_LIFE:u8 = 42; // no fixed address (its gonna be replaced inline at compilation time)
//have to declare the type by yourself
// all caps by standard
static Z:i32 = 123;
static mut Z_MUT:i32 = 123; // UNSAFE!!


fn scope_and_shadowing(){
        println!("scope_and_shadowing function");
        //scope and shadowing
        //let a = 123; //COMMENTED TO GET RID OF WARNING AT COMPILATION
        let a =1234; // overrides previous declaration

        println!("outside (before), a = {}", a);
        //unnamed code block
        {
                let b = 456;
                println!("inside, b = {}", b);
                let a = 777; //shadows the previous declaration
                println!("inside, a = {}", a);
        }
        println!("outside, a = {}", a);
        //println!("outside, b = {}", b); // cannot find value 'b' in this scope
}

fn operators(){
        println!("operators function");
        // ARITHMETIC
        let mut a = 2+3*4; // follows PEMDAS
        println!("a = {}", a);
        a = a+1; // "--" and "++" operators are not supported
        a-=2; // -= += *= /= and %= are allowed
        println!("updated a = {}", a);
        println!("remainder of {} / {} = {}", a, 3, (a%3));
        // there is no "power" operator but a built-in function
        let a_cubed = i32::pow(a,3);
        println!(" a = {}, a_cubed = {}", a, a_cubed);
        let b = 2.5;
        let b_cubed = f64::powi(b,3); //powi for integer exponent
        let b_to_pi = f64::powf(b,std::f64::consts::PI); //powf for float exponent
        println!(" {} cubed is {}", b, b_cubed);
        println!(" {} to the powe of pi is {}", b, b_to_pi);


        //BITWISE
        let c = 1 | 2;      // | OR & AND ^ XOR ! NOR
                                // 01 OR 10 == 11 == 3_10 (3 in base 10)
        println!("1|2 = {}", c);
        let two_to_10 = 1 << 10;// shift to the left
        println!("1 << 10 = {}", two_to_10);

        //LOGICAL
        let pi_less_4 = std::f64::consts::PI < 4.0; // less than
        // true
        // also > <= and >=
        println!("pi_less_4 = {}", pi_less_4);
        let x = 5;
        let x_is_five = x == 5; // equality
        // true
        println!("x_is_five = {}", x_is_five);



}


fn datatype_discovery(){
        println!("datatype_discovery function");
        let a:u8 = 123; // "let" creates a binding
        // u8 is 8 bits (1 byte)
        //u stands for unsigned ... values from 0 to 255
        //let another:i8 = 123; // signed 8bits, values from -127 to 128
        //COMMENTED TO GET RID OF WARNING AT COMPILATION

        println!("a = {}", a);
        //a = 456; // immutable variable ==> cant do

        let mut b:i8 = 0; // mutable variable!
        println!("b = {}", b);
        b = 42; // mutable variable!
        println!("b = {}", b);

        let mut c = 12345678; // will be guessed as signed 32-bits number
        // but IDEA guess and does it automatically
        println!("c = {} size = {} bytes", c, mem::size_of_val(&c));
        // getting the size of c with size_of_val not "size_val", using "&"
        c = -1; // checking its signed
        println!("c = {} after modification", c);
        // i8 u8 i16 u16 i32 u32 i64 u64

        let z:isize = 123; // integral datatype => size equal to the memory address of the system
        let size_of_z = mem::size_of_val(&z);
        println!("z = {}, takes up {} bytes, {}-bit OS",
                 z, size_of_z, size_of_z *8);

        let d = 'x'; // chars use single quotes ''
        println!("d = {} size = {} bytes", d, mem::size_of_val(&d));

        let e = 2.5; // double-precision float, 8 bytes or 64 bits
        println!("e = {} size = {} bytes", e, mem::size_of_val(&e));
        let e_prime:f32 = 2.5; // if we want float 32, 4 bytes or 32 bits
        println!("e_prime = {} size = {} bytes", e_prime, mem::size_of_val(&e_prime));

        // booleans (true or false)
        let g = false;
        println!("g = {} size = {} bytes", g, mem::size_of_val(&g));
        let f=4>0; // true
        println!("f is {}", f);
}

// first thing touched in the lecture
fn initial_main()
{
        println!("initial_main function");
        println!("test") // "println!"  is a macro cause of the "!"
}

pub fn types_and_variables(){
        println!("START OF main of mod types_and_variables;");
        initial_main();
        datatype_discovery();
        operators();
        scope_and_shadowing();
        println!("MEANING_OF_LIFE = {}", MEANING_OF_LIFE); // the string is needed first, no auto-conversion from u8
        println!("Z = {}", Z);
        unsafe{ // you promise you'll be careful :D
                println!("before change Z_MUT = {}", Z_MUT);
                Z_MUT = 777;
                println!("after change Z_MUT = {}", Z_MUT);
        }
        println!("END OF main of mod types_and_variables;");

}