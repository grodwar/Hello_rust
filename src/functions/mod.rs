#![allow(dead_code)]


fn print_value(value:i32)
{
    println!("value = {}", value);
}

fn increase(x: & mut i32)
{
    *x += 1;
}

fn product(x:i32,y:i32) ->  i32
{
    x*y // automatically returns cause there is no ";" at the end
}

pub fn function_and_arguments(){
    print_value(33);
    let mut z = 1;
    increase(& mut z);
    println!("z is now = {}",z);
    increase(& mut z);
    println!("z is now = {}",z);

    let a=3;
    let b=5;
    let p = product(a,b);
    println!("{}*{}={}",a,b,p);
}

///// METHODS
struct Point
{
    x: f64,
    y: f64

}

struct Line
{
    start: Point,
    end: Point
}
impl Line
{
    fn len(&self) ->f64//to be able to access its start and end point
    {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx*dx+dy*dy).sqrt()
    }
}

pub fn methods()
{
    let p = Point{x: 3.0, y:4.0};
    let p2 = Point{x: 5.0, y:10.0};
    let my_line = Line{start:p, end:p2};
    println!("length ={}", my_line.len());
}


///// CLOSURES

fn say_hello() { println!("Hello world!");}

pub fn closures()
{
    say_hello();
    let sh = say_hello;
    sh();
    // lets define a closure (an inline function (apparently))
    let plus_one = |x:i32| -> i32 {x+1};
    let a = 41;
    let b = plus_one(a);
    println!("{}+1={}",a,b);
    println!("{}+1={}",b,plus_one(b));

    //more interesting example
    let plus_two = |x|
        {
            let mut z = x;
            z+=2;
            z //returns z
        };
    println!("{}+2={}",b,plus_two(b));
    // part 2 of the example, with borrow issue
    // the issue occurs only if we try to use plus_two_bis after trying to change borrow_two
    // see the commented part at the end of this block
    //let mut two = 2; // we comment this to avoid some compile warning when not playing with this part
    let two = 2;

    let plus_two_bis = |x|
        {
            let mut z = x;
            z+=two;
            z //returns z
        };
    println!("(bis) {}+2={}",b,plus_two_bis(b));
    //let borrow_two = &mut two;//error[E0596]: cannot borrow `two` as mutable, as it is not declared as mutable
    //*borrow_two = 42; // cannot borrow `two` as mutable because it is also borrowed as immutable
    //two = 3; // cannot assign to `two` because it is borrowed
    //println!("(bis) {}+2={}",b,plus_two_bis(b));

    // T: by value
    // T&: by reference
    // &mut T: by mutable reference

    let plus_three= |x:&mut i32| *x += 3;
    let mut f = 12;
    println!("f={}",f);
    plus_three(&mut f);
    println!("f+3={}",f);

}


///// HIGHER ORDER FUNCTIONS
// functions that take function as parameter
// f(g) {let x = g();}
// functions that return function (referred often as generators)
// f() -> g


// not a higer order function
fn is_even(x:u32) -> bool
{
    x%2 == 0 //no ";" !!!
}

// this is a higher order function
fn greater_than(limit:u32)  -> impl Fn(u32) -> bool
{
    move |y|  y > limit //needs move because:
    //closure may outlive the current function, but it borrows `limit`, which is owned by the current function
    //@todo understand the "borrowing"
}

pub fn higher_order_functions(){
    //sum of all even squares < 500

    let limit = 500;
    let mut sum = 0;

    //inline function (closure) approach
    //let above_limit = |y| y > limit;
    //with the higher order function
    let above_limit = greater_than(limit);

    for i in 0.. { //from 0 to infinity (and beyond!)
        let isq = i*i;

        //if isq > limit // normal version
        if above_limit(isq)
        {
            break;
        }
        else if is_even(isq)
        {
            sum += isq;
        }
    }
    println!("loop sum = {}", sum);

    // more advanced version
    //working with sequence I guess
    let sum2 = (0..)
    .map(|x| x*x)
    .take_while(|&x| x <limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum+x);

    println!("hof sum = {}", sum2);


}