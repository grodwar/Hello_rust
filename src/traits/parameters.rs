use std::fmt::Debug;

#[derive(Debug)] // for version 3
struct Circle{
radius:f64,
}
#[derive(Debug)] // for version 3

struct Square{
    side:f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64
    {
        self.side*self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64
    {
         std::f64::consts::PI*self.radius*self.radius
    }

}

//version 1
fn print_info1(shape: impl Shape)
{
    println!("(v1) The area is {}",shape.area());
}

//version 2 and 3
fn print_info2(shape: impl Shape + Debug)
{
    println!("(v2) debug: {:?}",shape);
    println!("(v2) The area is {}",shape.area());
}

//version 4 trade bound syntax
//benefits if we have multiple arguments
fn print_info4<T: Shape + Debug>(shape:T)
{
    println!("(v4) debug: {:?}",shape);
    println!("(v4) The area is {}",shape.area());
}

//version 5
fn print_info5<T>(shape:T)
    where T:  Shape + Debug
{
    println!("(v5) debug: {:?}",shape);
    println!("(v5) The area is {}",shape.area());
}

pub fn parameters(){
    let c = Circle{radius:2.0};
    print_info1(c);
    //version 2 and 3
    let c2 = Circle{radius:2.0};
    print_info2(c2); //doesn't implement `std::fmt::Debug' before I update it
    //version 4
    let c4 = Circle{radius:2.0};
    print_info4(c4);
    //version 5
    let c5 = Circle{radius:2.0};
    print_info4(c5);

}